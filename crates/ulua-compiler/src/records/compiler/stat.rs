//! `Compiler` 语句编译与循环控制流：`compile_stat` 分发、局部/赋值/函数声明语句、
//! 循环边界与跳转补丁（对照 cpp `Compiler.cpp` 的 compileStat* / beginLoop 段）。
use core::{cmp::max, ptr::from_mut, slice::from_ref};

use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_call::AstExprCall,
    ast_expr_global::AstExprGlobal,
    ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal,
    ast_node::AstNode,
    ast_stat::AstStat,
    ast_stat_assign::AstStatAssign,
    ast_stat_block::AstStatBlock,
    ast_stat_break::AstStatBreak,
    ast_stat_class::AstStatClass,
    ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_continue::AstStatContinue,
    ast_stat_expr::AstStatExpr,
    ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction,
    ast_stat_if::AstStatIf,
    ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction,
    ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn,
    ast_stat_while::AstStatWhile,
  },
  rtti::{AstNodeClass, ast_node_try_as_ptr, ast_node_try_as_ptr_mut},
  visit::ast_expr_visit,
};
use ulua_bytecode::{
  methods::bytecode_builder_get_string_hash::bytecode_builder_get_string_hash,
  records::{class_shape::ClassShape, string_ref::StringRef},
};
use ulua_common::{
  enums::{luau_bytecode_type::LuauBytecodeType, luau_opcode::LuauOpcode},
  fflag,
  fflag::DebugLuauUserDefinedClasses,
  fint::{LuauCompileLoopUnrollThreshold, LuauCompileLoopUnrollThresholdMaxBoost},
  macros::luau_assert::LUAU_ASSERT,
  records::{dense_hash_set::DenseHashSet, variant::Variant2},
};

use crate::{
  enums::{
    kind::Kind,
    type_compiler::{
      Type, Type as LoopJumpType,
      Type::{Break, Continue},
    },
  },
  functions::{
    always_terminates::always_terminates,
    ast_slot_ref::{ast_slot_is, ast_slot_ref, ast_slot_try_as},
    c_const::cnum,
    compute_cost::compute_cost,
    cost_model::model_cost,
    get_builtin::get_builtin,
    get_trip_count::get_trip_count,
    is_constant::{is_constant_false, is_constant_true},
    sref_compiler::{sref_ast_array_u8, sref_ast_name},
    undo_changes_constant_folding::{undo_changes_expr, undo_changes_local},
  },
  records::{
    assignment::Assignment,
    compile_error::{CompileError, ERR_EXCEEDED_JUMP_DISTANCE_LIMIT},
    compiler::{
      Compiler, K_COST_PERCENT_SCALE, K_DEFAULT_ALLOC_PC, K_GETIMPORT_FLAG, K_INVALID_REG,
      K_MAX_K_CONST_INDEX, node_downcast,
    },
    constant::Constant,
    l_value::LValue,
    r#loop::Loop,
    loop_jump::LoopJump,
    node::Node,
    reg_scope::RegScope,
    undefined_local_visitor::UndefinedLocalVisitor,
  },
};

#[inline]
fn node_downcast_mut<T: AstNodeClass + 'static>(node: *mut AstStat) -> *mut T {
  unsafe {
    ast_node_try_as_ptr_mut::<T>(node).expect("类索引命中与 RTTI 下转同一判定，此分支不可达")
  }
}

impl Compiler {
  /// C++ `compileStat`：按动态类型分发到各语句编译器。
  pub(crate) fn compile_stat(&mut self, node: *mut AstStat) {
    // Safety: 调用方保证 node 指向 arena 存活 AstStat（同 cpp 直接解引用），
    // 基类借用在此一次建立，后续不再散点解引用
    let base = &ast_slot_ref(node)
      .expect("compile_stat 入口契约：node 非空且指向 arena 存活 AstStat")
      .base;
    self.set_debug_line_ast_node(base);
    if self.options.coverage_level >= 1 && self.needs_coverage(base) {
      self.bc_mut().emit_abc(LuauOpcode::LOP_COVERAGE, 0, 0, 0);
    }

    match base.class_index {
      AstStatBlock::CLASS_INDEX => {
        let stat = node_downcast::<AstStatBlock>(base);
        let _rs = self.reg_scope();
        let old_locals = self.local_stack.len();
        if fflag::LuauExportValueSyntax.get() {
          self.block_depth += 1;
        }
        for body_stat in stat.body.iter_nodes() {
          self.compile_stat(body_stat.as_ptr());
          if always_terminates(&self.constants, body_stat.as_ptr()) {
            break;
          }
        }
        if fflag::LuauExportValueSyntax.get() {
          self.block_depth -= 1;
        }
        self.close_locals(old_locals);
        self.pop_locals(old_locals);
      }
      AstStatIf::CLASS_INDEX => self.compile_stat_if(node_downcast(base)),
      AstStatWhile::CLASS_INDEX => self.compile_stat_while(node_downcast(base)),
      AstStatRepeat::CLASS_INDEX => {
        let stat = unsafe { &mut *node_downcast_mut::<AstStatRepeat>(node) };
        self.compile_stat_repeat(stat);
      }
      AstStatBreak::CLASS_INDEX => {
        LUAU_ASSERT!(!self.loops.is_empty());
        self.close_locals(self.loops.last().unwrap().local_offset);
        let label = self.bc().emit_label();
        self.bc_mut().emit_ad(LuauOpcode::LOP_JUMP, 0, 0);
        self.loop_jumps.push(LoopJump {
          r#type: Break,
          label,
        });
      }
      AstStatContinue::CLASS_INDEX => {
        let stat = unsafe { &mut *node_downcast_mut::<AstStatContinue>(node) };
        LUAU_ASSERT!(!self.loops.is_empty());
        if self.loops.last().unwrap().continue_used.is_none() {
          self.loops.last_mut().unwrap().continue_used = Some(Node::from_mut(stat));
        }
        self.close_locals(self.loops.last().unwrap().local_offset_continue);
        let label = self.bc().emit_label();
        self.bc_mut().emit_ad(LuauOpcode::LOP_JUMP, 0, 0);
        self.loop_jumps.push(LoopJump {
          r#type: Continue,
          label,
        });
      }
      AstStatReturn::CLASS_INDEX => {
        let stat = node_downcast::<AstStatReturn>(base);
        if self.options.optimization_level >= 2 && !self.inline_frames.is_empty() {
          self.compile_inline_return(stat, false);
        } else {
          self.compile_stat_return(stat);
        }
      }
      AstStatExpr::CLASS_INDEX => {
        let stat = node_downcast::<AstStatExpr>(base);
        // expr 已句柄化（node_handle::Node）：判型门面与 compile_expr_side 的既有
        // 裸指针 API 经 as_ptr 桥接（arena 存活前提由 Node 供给，无需调用点 unsafe）。
        if let Some(expr) = unsafe { ast_node_try_as_ptr_mut::<AstExprCall>(stat.expr.as_ptr()) } {
          self.compile_expr_call(from_mut(expr), self.reg_top as u8, 0, false, false);
        } else {
          self.compile_expr_side(stat.expr.as_ptr());
        }
      }
      AstStatLocal::CLASS_INDEX => {
        let stat = node_downcast::<AstStatLocal>(base);
        if fflag::LuauExportValueSyntax.get() {
          for &var in stat.vars.iter() {
            self.check_exported_local(unsafe { &mut *var }, &base.location);
          }
        }
        self.compile_stat_local(stat);
      }
      AstStatFor::CLASS_INDEX => {
        let stat = unsafe { &mut *node_downcast_mut::<AstStatFor>(node) };
        self.compile_stat_for(stat);
      }
      AstStatForIn::CLASS_INDEX => {
        let stat = unsafe { &mut *node_downcast_mut::<AstStatForIn>(node) };
        self.compile_stat_for_in(stat);
      }
      AstStatAssign::CLASS_INDEX => {
        let stat = unsafe { &mut *node_downcast_mut::<AstStatAssign>(node) };
        self.compile_stat_assign(from_mut(stat));
      }
      AstStatCompoundAssign::CLASS_INDEX => {
        let stat = unsafe { &mut *node_downcast_mut::<AstStatCompoundAssign>(node) };
        self.compile_stat_compound_assign(from_mut(stat));
      }
      AstStatFunction::CLASS_INDEX => self.compile_stat_function(node_downcast(base)),
      AstStatLocalFunction::CLASS_INDEX => {
        let stat = unsafe { &mut *node_downcast_mut::<AstStatLocalFunction>(node) };
        // name 已句柄化为 Node<AstLocal>（类型层非空）：可变借用沿 `&mut stat`
        // 经 get_mut 传递，不再手写裸指针解引用（同相邻 For/ForIn 臂形态）。
        let name = stat.name.get_mut();
        if fflag::LuauExportValueSyntax.get() && name.is_exported {
          self.check_exported_local(name, &base.location);
          self.ensure_export_table(base);
          let _rs = self.reg_scope();
          let var = self.alloc_reg(base, 1);
          self.compile_expr_function(stat.func, var);
          let name_ref = sref_ast_name(name.name);
          let cid = self.bc_mut().add_constant_string(name_ref.clone());
          self.check_constant(cid, &name.location);
          let table_reg = self.get_export_table_reg(base);
          self.bc_mut().emit_abc(
            LuauOpcode::LOP_SETTABLEKS,
            var,
            table_reg,
            bytecode_builder_get_string_hash(name_ref) as u8,
          );
          self.bc_mut().emit_aux(cid as u32);
        } else {
          let var = self.alloc_reg(base, 1);
          self.push_local(name, var, K_DEFAULT_ALLOC_PC);
          if fflag::LuauExportValueSyntax.get() {
            self.check_exported_local(name, &base.location);
          }
          self.compile_expr_function(stat.func, var);
          let debugpc = self.bc().get_debug_pc();
          self.locals.get_or_insert(Node::from_ref(name)).debugpc = debugpc;
        }
      }
      AstStatClass::CLASS_INDEX if fflag::DebugLuauUserDefinedClasses.get() => {
        let stat = unsafe { &mut *node_downcast_mut::<AstStatClass>(node) };
        self.compile_class_declaration(stat);
      }
      _ => {}
    }
  }

  /// 对应 cpp `Compiler::compileStatAssign`。原裸指针契约 fn 已 safe 化：
  /// `stat` 由 `compile_stat` 分发器判型后经 `from_mut` 交还（arena 存活、非空、
  /// 编译期地址稳定），`vars`/`values` 的 `AstArray` 槽位受各自 `size` 界约束，
  /// 越界即扫描 arena 脏内存——该前提由分发器接线构造性保证，不再是调用方须
  /// 自证的跨函数契约。只读遍历全部经槽位门面；残余窄独占借用仅 vars/values
  /// 表达式节点的编译期临时字段写穿。
  pub(crate) fn compile_stat_assign(&mut self, stat: *mut AstStatAssign) {
    {
      // 门面无借用解引用：`stat` 是分发器判型后 from_mut 交还的合法
      // `AstStatAssign`（arena 分配、非空、地址稳定）。
      let stat_ref =
        ast_slot_ref(stat).expect("compile_stat_assign 契约：stat 为分发器判型的存活节点");
      let mut rs = self.reg_scope();

      if stat_ref.vars.size == 1 && stat_ref.values.size == 1 {
        // 双 size==1 守卫下首槽经 as_slice 界内取回，无需裸 data 解引用
        let var_expr = stat_ref.vars.as_slice()[0];
        let value_expr = stat_ref.values.as_slice()[0];
        // var_expr 为 vars 首槽的存活 AstExpr 子指针，rs 为宿主 RegScope。
        let var = self.compile_l_value(var_expr, &mut rs);
        if var.kind == Kind::Local {
          // Safety: value_expr 为存活 AstExpr 子指针；&mut 写穿限于该节点编译期临时字段。
          self.compile_expr(unsafe { &mut *value_expr }, var.reg, false);
        } else {
          let reg = self.compile_expr_auto(value_expr, &mut rs);
          // 门面解引用：仅读存活 var_expr 节点的基类字段。
          self.set_debug_line_ast_node(
            &ast_slot_ref(var_expr)
              .expect("var_expr 为 vars 首槽的存活 AstExpr 子指针")
              .base,
          );
          self.compile_assign(&var, reg, Some(var_expr));
        }
        return;
      }

      let mut vars = Vec::with_capacity(stat_ref.vars.size);
      for &var_expr in stat_ref.vars.iter() {
        vars.push(Assignment {
          // 各 var_expr 均为 vars 数组内的存活 AstExpr 子指针，rs 为宿主 RegScope。
          lvalue: self.compile_l_value(var_expr, &mut rs),
          conflict_reg: K_INVALID_REG,
          value_reg: K_INVALID_REG,
        });
      }

      // stat 为契约保证的存活 AstStatAssign，cast 到基类指针仍是同一 arena
      // 节点首地址；vars/values 切片受 size 界约束。
      self.resolve_assign_conflicts(stat.cast::<AstStat>(), &mut vars, &stat_ref.values);

      // take 取较短一侧，等价于 C++ 的 `min(vars.size, values.size)` 循环
      for (i, &value) in stat_ref.values.iter().take(vars.len()).enumerate() {
        if i + 1 == stat_ref.values.size && stat_ref.vars.size > stat_ref.values.size {
          let rest = (stat_ref.vars.size - stat_ref.values.size + 1) as u32;
          let temp = self.alloc_reg(&stat_ref.base.base, rest);
          // Safety: value 为 values 数组尾槽的存活 AstExpr 子指针；&mut 写穿限于该节点。
          self.compile_expr_temp_n(unsafe { &mut *value }, temp, rest as u8, true);
          for j in i..stat_ref.vars.size {
            vars[j].value_reg = temp + (j - i) as u8;
          }
        } else {
          let var = &mut vars[i];
          if var.lvalue.kind == Kind::Local {
            var.value_reg = if var.conflict_reg == K_INVALID_REG {
              var.lvalue.reg
            } else {
              var.conflict_reg
            };
            // Safety: value 为 values 数组内的存活 AstExpr 子指针；&mut 写穿限于该节点。
            self.compile_expr(unsafe { &mut *value }, var.value_reg, false);
          } else {
            var.value_reg = self.compile_expr_auto(value, &mut rs);
          }
        }
      }

      for &value in stat_ref.values.iter().skip(stat_ref.vars.size) {
        self.compile_expr_side(value);
      }

      for (i, var) in vars.iter().enumerate() {
        LUAU_ASSERT!(var.value_reg != K_INVALID_REG);
        if var.lvalue.kind != Kind::Local {
          self.set_debug_line_location(&var.lvalue.location);
          // 越界=左值缺失（values 多于 vars 的 cpp null 补位）→ 切片 get()/Option
          let target_expr = stat_ref.vars.as_slice().get(i).copied();
          self.compile_assign(&var.lvalue, var.value_reg, target_expr);
        }
      }

      for var in vars {
        if var.lvalue.kind == Kind::Local && var.value_reg != var.lvalue.reg {
          self
            .bc_mut()
            .emit_abc(LuauOpcode::LOP_MOVE, var.lvalue.reg, var.value_reg, 0);
        }
      }
    }
  }

  /// 对应 cpp `Compiler::compileStatCompoundAssign`。原裸指针契约 fn 已 safe 化：
  /// `stat` 由分发器判型后 arena 存活交还（非空、地址稳定）；`var`/`value`
  /// 已是 [`ulua_ast::records::node_handle::Node`]（非空证明内建），只读消费——
  /// `var` 交 `compile_l_value`（其 RTTI 分支要求 var 为 l-value 形状：
  /// Local/IndexName/IndexExpr，由 parser 接线保证）。
  /// 残余窄独占借用仅为 Concat 实参节点的临时字段写穿。
  pub(crate) fn compile_stat_compound_assign(&mut self, stat: *mut AstStatCompoundAssign) {
    {
      // 门面无借用解引用：`stat` 为分发器传入的合法 `AstStatCompoundAssign`
      // （分发器契约：arena 分配、非空、地址稳定）。
      let stat_ref =
        ast_slot_ref(stat).expect("compile_stat_compound_assign 契约：stat 为分发器判型的存活节点");
      let mut rs = self.reg_scope();
      // stat_ref.var 已句柄化（node_handle::Node）：as_ptr 仅透传同一 arena 地址给
      // 仍以裸指针为键的 compile/hint 门面，rs 为宿主 RegScope。
      let var = self.compile_l_value(stat_ref.var.as_ptr(), &mut rs);
      let target = if var.kind == Kind::Local {
        var.reg
      } else {
        self.alloc_reg(&stat_ref.base.base, 1)
      };

      match stat_ref.op {
        AstExprBinaryOp::Add
        | AstExprBinaryOp::Sub
        | AstExprBinaryOp::Mul
        | AstExprBinaryOp::Div
        | AstExprBinaryOp::FloorDiv
        | AstExprBinaryOp::Mod
        | AstExprBinaryOp::Pow => {
          if var.kind != Kind::Local {
            self.compile_l_value_use(&var, target, false, Some(stat_ref.var.as_ptr()));
          }
          // value 已句柄化：`.get()` 直供只读常量折叠，原 ast_slot_ref+expect 判空门面消失。
          let rc = self.get_constant_number(stat_ref.value.get());
          if (0..K_MAX_K_CONST_INDEX).contains(&rc) {
            let op = self.get_binary_op_arith(stat_ref.op, true);
            self.bc_mut().emit_abc(op, target, target, rc as u8);
          } else {
            let rr = self.compile_expr_auto(stat_ref.value.as_ptr(), &mut rs);
            let op = self.get_binary_op_arith(stat_ref.op, false);
            self.bc_mut().emit_abc(op, target, target, rr);
            if var.kind != Kind::Local {
              self.hint_temporary_reg_type(
                stat_ref.var.as_ptr(),
                target as i32,
                LuauBytecodeType::LBC_TYPE_NUMBER,
                1,
              );
            }
            self.hint_number_reg(stat_ref.value.as_ptr(), rr);
          }
        }
        AstExprBinaryOp::Concat => {
          let mut args = vec![stat_ref.value.as_ptr()];
          self.unroll_concats(&mut args);
          let regs = self.alloc_reg(&stat_ref.base.base, (1 + args.len()) as u32);
          self.compile_l_value_use(&var, regs, false, Some(stat_ref.var.as_ptr()));
          for (i, &arg) in args.iter().enumerate() {
            // Safety: args 为同树存活 AstExpr 子指针；&mut 写穿限于节点临时字段。
            if fflag::LuauCompileConcatTargetTop.get() {
              self.compile_expr_temp_top(unsafe { &mut *arg }, regs + 1 + i as u8);
            } else {
              self.compile_expr(unsafe { &mut *arg }, regs + 1 + i as u8, true);
            }
          }
          self.bc_mut().emit_abc(
            LuauOpcode::LOP_CONCAT,
            target,
            regs,
            regs + args.len() as u8,
          );
        }
        _ => LUAU_ASSERT!(false),
      }

      if var.kind != Kind::Local {
        self.compile_assign(&var, target, Some(stat_ref.var.as_ptr()));
      }
    }
  }

  /// `stat_ref` 由分发器 `&mut` 交接证明存活且独占；`from`/`to`/`var`/`body`
  /// 已句柄化为 Node（parser 非空由类型层承载），`step` 落可空 OptNode 显式
  /// 判空后才解借用；`var` 交 `push_local` 登记，须长寿于本次编译。
  pub(crate) fn compile_stat_for(&mut self, stat_ref: &mut AstStatFor) {
    let _rs = self.reg_scope();

    if self.options.optimization_level >= 2
      && self.is_constant(stat_ref.to)
      && self.is_constant(stat_ref.from)
      && (stat_ref.step.is_none() || self.is_constant(stat_ref.step.as_ptr()))
    {
      // C++ 传 FInt::LuauCompileLoopUnrollThreshold / ...MaxBoost；旧硬编码
      // 100/100 既无视配置默认值也无视测试期 ScopedFastInt 覆写，导致迭代数
      // 超过阈值的循环（如阈值 25 下的 for i=1,100）被错误展开。
      if self.try_compile_unrolled_for(
        stat_ref,
        LuauCompileLoopUnrollThreshold.get(),
        LuauCompileLoopUnrollThresholdMaxBoost.get(),
      ) {
        return;
      }
    }

    let (old_locals, old_jumps) = self.begin_loop();

    let regs = self.alloc_reg(&stat_ref.base.base, 3);
    let varregallocpc = self.bc().get_debug_pc();
    let mut varreg = regs + 2;

    if let Some(il) = self.variables.find(&stat_ref.var.into())
      && il.written
    {
      varreg = self.alloc_reg(&stat_ref.base.base, 1);
    }

    // from/to/step 已句柄化：可变借用沿 `&mut stat_ref` 逐级传递（可空槽经
    // get_mut 落 Option），&mut 借用只覆盖各次分发调用。
    self.compile_expr(stat_ref.from.get_mut(), regs + 2, true);
    self.compile_expr(stat_ref.to.get_mut(), regs, true);

    if let Some(step) = stat_ref.step.get_mut() {
      self.compile_expr(step, regs + 1, true);
    } else {
      self
        .bc_mut()
        .emit_abc(LuauOpcode::LOP_LOADN, regs + 1, 1, 0);
    }

    let for_label = self.bc().emit_label();
    self.bc_mut().emit_ad(LuauOpcode::LOP_FORNPREP, regs, 0);
    let loop_label = self.bc().emit_label();

    if varreg != regs + 2 {
      self
        .bc_mut()
        .emit_abc(LuauOpcode::LOP_MOVE, varreg, regs + 2, 0);
    }

    // var 已句柄化为 Node（parser 分配的存活 AstLocal，非空由类型层承载），
    // `.get()` 直出引用；varreg 由 alloc_reg 预留。
    self.push_local(stat_ref.var.get(), varreg, varregallocpc);
    self.compile_stat(stat_ref.body.as_ptr().cast::<AstStat>());

    self.close_locals(old_locals);
    self.pop_locals(old_locals);
    self.set_debug_line_ast_node(&stat_ref.base.base);

    let cont_label = self.bc().emit_label();
    let back_label = self.bc().emit_label();
    self.bc_mut().emit_ad(LuauOpcode::LOP_FORNLOOP, regs, 0);
    let end_label = self.bc().emit_label();

    self.patch_jump(&stat_ref.base.base, for_label, end_label);
    self.patch_jump(&stat_ref.base.base, back_label, loop_label);

    self.end_loop(&stat_ref.base.base, old_jumps, end_label, cont_label);
  }

  /// `values`/`vars` 槽位在节点 `size` 界内；`vars` 各 `*mut AstLocal` 交
  /// `push_local` 登记，须长寿于本编译；快路径 RTTI 命中后才读 `call.func`。
  pub(crate) fn compile_stat_for_in(&mut self, stat_ref: &AstStatForIn) {
    let _rs = self.reg_scope();

    let (old_locals, old_jumps) = self.begin_loop();

    let regs = self.alloc_reg(&stat_ref.base.base, 3);

    self.compile_expr_list_temp(&stat_ref.values, regs, 3, true);

    let vars = self.alloc_reg(&stat_ref.base.base, max(stat_ref.vars.size as u32, 2));
    LUAU_ASSERT!(vars == regs + 3);
    let vars_alloc_pc = self.bc().get_debug_pc();

    let mut skip_op = LuauOpcode::LOP_FORGPREP;

    if self.options.optimization_level >= 1 && stat_ref.vars.size <= 2 {
      // 门面判型+下转：values[0] 是 parser 接线进 arena 的存活节点指针，命中 AstExprCall 才返回引用。
      if stat_ref.values.size == 1
        && let Some(call) = ast_slot_try_as::<AstExprCall, _>(stat_ref.values.as_slice()[0])
      {
        // C++ 传给 getBuiltin 的是 CALL 的 `func`（即 `ipairs`/`pairs` 引用），
        // 而非整个调用表达式。旧模型把 `ipairs(t)` 整体传入，永远匹配不上
        // 内建，循环退回通用 FORGPREP。
        let builtin = get_builtin(call.func.into(), &self.globals, &self.variables);

        if builtin.is_global(b"ipairs") {
          skip_op = LuauOpcode::LOP_FORGPREP_INEXT;
        } else if builtin.is_global(b"pairs") {
          skip_op = LuauOpcode::LOP_FORGPREP_NEXT;
        }
      } else if stat_ref.values.size == 2 && !self.getfenv_used && !self.setfenv_used {
        // cpp: 使用 getfenv/setfenv 时 `next` 可能被换掉，禁走 FORGPREP_NEXT 快路径
        let builtin = get_builtin(
          stat_ref.values.as_slice()[0].into(),
          &self.globals,
          &self.variables,
        );

        if builtin.is_global(b"next") {
          skip_op = LuauOpcode::LOP_FORGPREP_NEXT;
        }
      }
    }

    let skip_label = self.bc().emit_label();

    self.bc_mut().emit_ad(skip_op, regs, 0);

    let loop_label = self.bc().emit_label();

    for (i, &var) in stat_ref.vars.iter().enumerate() {
      // 门面解引用：var 为 parser 接线存活 AstLocal，寄存器槽由 alloc_reg 预留。
      self.push_local(
        ast_slot_ref(var).expect("vars 槽由 parser 保证存活 AstLocal"),
        (vars + i as u8) as u8,
        vars_alloc_pc,
      );
    }

    // body 已句柄化为 Node（非空由类型层承载），as_ptr+cast 桥交指针形态门面。
    self.compile_stat(stat_ref.body.as_ptr().cast::<AstStat>());

    self.close_locals(old_locals);
    self.pop_locals(old_locals);

    self.set_debug_line_ast_node(&stat_ref.base.base);

    let cont_label = self.bc().emit_label();
    let back_label = self.bc().emit_label();

    self.bc_mut().emit_ad(LuauOpcode::LOP_FORGLOOP, regs, 0);
    self.bc_mut().emit_aux(
      if skip_op == LuauOpcode::LOP_FORGPREP_INEXT {
        K_GETIMPORT_FLAG
      } else {
        0
      } | stat_ref.vars.size as u32,
    );

    let end_label = self.bc().emit_label();

    self.patch_jump(&stat_ref.base.base, skip_label, back_label);
    self.patch_jump(&stat_ref.base.base, back_label, loop_label);

    self.end_loop(&stat_ref.base.base, old_jumps, end_label, cont_label);
  }

  /// 把一批条件跳变标签整体转为 break/continue 跳转压入 loop_jumps
  /// （cpp compileStatIf 两处 for-push 循环的复用体）。
  fn push_loop_jumps(&mut self, r#type: Type, jumps: Vec<usize>) {
    self
      .loop_jumps
      .extend(jumps.into_iter().map(|label| LoopJump { r#type, label }));
  }

  /// `thenbody`/`elsebody` 已句柄化：非空由 `Node` 类型端兑现，`elsebody` 经
  /// `OptNode::as_ptr` 折回既有裸指针 API（null 即「无 else」），全程只读。
  pub(crate) fn compile_stat_if(&mut self, stat: &AstStatIf) {
    // thenbody 由 Node 类型端兑现非空，`get` 交出只读引用供 continue 抽取。
    let then_ref = stat.thenbody.get();
    let then_ptr = stat.thenbody.as_ptr().cast::<AstStat>();
    let else_ptr = stat.elsebody.as_ptr();
    let cond_ptr = stat.condition.as_ptr();

    if is_constant_false(&self.constants, stat.condition.into()) {
      if stat.elsebody.is_some() {
        self.compile_stat(else_ptr);
      }
      return;
    }

    // 门面判型+下转：condition 经 `as_ptr` 折回 arena 存活表达式指针，命中 AstExprBinary 才继续。
    if let Some(cand) = ast_slot_try_as::<AstExprBinary, _>(cond_ptr)
      && cand.op == AstExprBinaryOp::And
      && is_constant_false(&self.constants, cand.right.into())
    {
      // left 已句柄化；compile_expr_side 为既有裸指针 API，经 as_ptr 桥接。
      self.compile_expr_side(cand.left.as_ptr());
      if stat.elsebody.is_some() {
        self.compile_stat(else_ptr);
      }
      return;
    }

    if stat.elsebody.is_none()
      && self.is_stat_break(then_ptr)
      && !self.are_locals_captured(self.loops.last().unwrap().local_offset)
    {
      let else_jump = self.compile_condition_value(cond_ptr, None, true);
      self.push_loop_jumps(Type::Break, else_jump);
      return;
    }

    let continue_statement = self.extract_stat_continue(then_ref);
    if stat.elsebody.is_none()
      && continue_statement.is_some()
      && !self.are_locals_captured(self.loops.last().unwrap().local_offset_continue)
    {
      if self.loops.last().unwrap().continue_used.is_none() {
        self.loops.last_mut().unwrap().continue_used = continue_statement;
      }
      let else_jump = self.compile_condition_value(cond_ptr, None, true);
      self.push_loop_jumps(Type::Continue, else_jump);
      return;
    }

    let else_jump = self.compile_condition_value(cond_ptr, None, false);
    self.compile_stat(then_ptr);

    if stat.elsebody.is_some() && !else_jump.is_empty() {
      if always_terminates(&self.constants, then_ptr) {
        let else_label = self.bc().emit_label();
        self.compile_stat(else_ptr);
        self.patch_jumps(&stat.base.base, &else_jump, else_label);
      } else {
        let then_label = self.bc().emit_label();
        self.bc_mut().emit_ad(LuauOpcode::LOP_JUMP, 0, 0);
        let else_label = self.bc().emit_label();
        self.compile_stat(else_ptr);
        let end_label = self.bc().emit_label();
        self.patch_jumps(&stat.base.base, &else_jump, else_label);
        self.patch_jump(&stat.base.base, then_label, end_label);
      }
    } else {
      let end_label = self.bc().emit_label();
      self.patch_jumps(&stat.base.base, &else_jump, end_label);
    }
  }

  /// 编译 `local a = ...` 语句。槽位判空/解引用统一走只读门面 [`ast_slot_ref`]，
  /// `*mut AstLocal` 只保留为 `variables`/`local_stack` 的地址键（句柄模型）。
  ///
  /// 未移植：cpp:4143-4153 在 `FFlag::LuauCompileMoveElision`（上游默认 false，
  /// Common.h:139 单参宏恒 false；开启前 `InlineFrame.resultLocal` 恒 nullptr，该
  /// 分支双重死码）下让 `local x = <内联调用>` 直接复用 target 寄存器。本移植全线
  /// 按 flag-off 臂实现，等价上游默认态（b23-moveelision-sync 调研裁定，移植清单见
  /// [`crate::records::inline_frame`] 文档注记）。
  pub(crate) fn compile_stat_local(&mut self, stat_ref: &AstStatLocal) {
    if self.options.optimization_level >= 1
      && self.options.debug_level <= 1
      && self.are_locals_redundant(stat_ref)
    {
      return;
    }

    if self.options.optimization_level >= 1 && stat_ref.vars.size == 1 && stat_ref.values.size == 1
    {
      // 上式已验 size == 1：两处首槽经 as_slice 界内取回，无需 unsafe
      let value = stat_ref.values.as_slice()[0];
      let var_slot = stat_ref.vars.as_slice()[0];

      if let (Some(re), Some(var)) = (
        self.get_expr_local(value).map(|node| node.borrow()),
        ast_slot_ref(var_slot),
      ) {
        // cpp `re->local` 槽位已句柄化恒非空（旧「空槽折叠」分支类型端不可达）。
        let rv_slot = re.local;
        let lv_written = self
          .variables
          .find(&var_slot.into())
          .is_some_and(|lv| lv.written);
        let rv_written = self
          .variables
          .find(&rv_slot.into())
          .is_some_and(|rv| rv.written);
        let reg = self.get_expr_local_reg(value);
        // rv_slot 已句柄化恒非空：.get() 安全借用读 is_exported。
        let exported_free = !var.is_exported && !rv_slot.get().is_exported;

        if reg >= 0 && !lv_written && !rv_written && exported_free {
          let allocpc = self.bc().get_debug_pc();
          self.push_local(var, reg as u8, allocpc);
          return;
        }
      }
    }

    let vars = self.alloc_reg(&stat_ref.base.base, stat_ref.vars.size as u32);
    let allocpc = self.bc().get_debug_pc();

    self.compile_expr_list_temp(&stat_ref.values, vars, stat_ref.vars.size as u8, true);

    for (i, &slot) in stat_ref.vars.iter().enumerate() {
      // 槽位为 parser 接线的存活 AstLocal（cpp 处直接解引用）：空槽绝对不可触发，
      // 与其静默跳过一个变量（会让 local 栈与寄存器错位）不如显式拒绝。
      let local = ast_slot_ref(slot).expect("AstStatLocal::vars slot must be non-null");

      if fflag::LuauExportValueSyntax.get() && local.is_exported {
        self.ensure_export_table(&stat_ref.base.base);

        let name_ref = sref_ast_name(local.name);
        let cid = self.bc_mut().add_constant_string(name_ref.clone());
        self.check_constant(cid, &local.location);

        let table_reg = self.get_export_table_reg(&stat_ref.base.base);
        self.bc_mut().emit_abc(
          LuauOpcode::LOP_SETTABLEKS,
          vars + i as u8,
          table_reg,
          bytecode_builder_get_string_hash(name_ref) as u8,
        );
        self.bc_mut().emit_aux(cid as u32);
      } else {
        self.push_local(local, vars + i as u8, allocpc);
      }
    }
  }

  /// `stat_ref` 由分发器 `&mut` 交接证明存活且独占；`body`/`condition` 已句柄化
  /// 为 Node（parser 非空由类型层承载），`get`/`get_mut` 直出借用，as_ptr 仅作
  /// 既有裸指针 API 的桥接；循环回跳 patch 依赖 `loop_jumps`/`local_stack` 水位
  /// 与 self 当前编译帧一致。
  pub(crate) fn compile_stat_repeat(&mut self, stat_ref: &mut AstStatRepeat) {
    let body = stat_ref.body.get();

    let (old_locals, old_jumps) = self.begin_loop();

    let loop_label = self.bc().emit_label();

    // cpp `RegScope rs(this)`：裸构造等价，但统一走构造函数便于维护
    let _rs = self.reg_scope();

    let mut continue_validated = false;
    let mut condition_locals = 0;

    for (i, body_stat) in body.body.iter_nodes().enumerate() {
      self.compile_stat(body_stat.as_ptr());

      self.loops.last_mut().unwrap().local_offset_continue = self.local_stack.len();

      if let Some(continue_used) = self.loops.last().unwrap().continue_used
        && !continue_validated
      {
        self.validate_continue_until(
          continue_used.cast::<AstStat>().borrow(),
          stat_ref.condition.get_mut(),
          body,
          i + 1,
        );
        continue_validated = true;
        condition_locals = self.local_stack.len();
      }
    }

    if continue_validated {
      // continue_validated 为真仅发生在逐条验证体内语句时，故 body 必非空；
      // 经 iter().last() 取尾语句，取代裸 data.add(size-1) 三重解引用
      // 尾语句槽已句柄化（Nodes）：非空由构造端证明，`get` 引用免门面判空。
      if let Some(last_stat) = body.body.iter_nodes().last() {
        self.set_debug_line_end(&last_stat.base);
      }

      self.close_locals(condition_locals);
      self.pop_locals(condition_locals);
    }

    let cont_label = self.bc().emit_label();

    let end_label;

    self.set_debug_line_ast_node(&stat_ref.condition.get().base);

    if is_constant_true(&self.constants, stat_ref.condition.into()) {
      self.close_locals(old_locals);

      end_label = self.bc().emit_label();
    } else {
      let skip_jump = self.compile_condition_value(stat_ref.condition.as_ptr(), None, true);

      self.close_locals(old_locals);

      let back_label = self.bc().emit_label();

      self.bc_mut().emit_ad(LuauOpcode::LOP_JUMPBACK, 0, 0);

      let skip_label = self.bc().emit_label();

      self.close_locals(old_locals);

      end_label = self.bc().emit_label();

      self.patch_jump(&stat_ref.base.base, back_label, loop_label);
      self.patch_jumps(&stat_ref.base.base, &skip_jump, skip_label);
    }

    self.pop_locals(old_locals);

    self.end_loop(&stat_ref.base.base, old_jumps, end_label, cont_label);
  }

  /// `list` 子表达式经 `iter()` 界内访问；返回寄存器区间由内部 alloc 保证，
  /// 须处于当前窗口。
  pub(crate) fn compile_stat_return(&mut self, stat_ref: &AstStatReturn) {
    {
      if stat_ref.list.size >= 255 {
        CompileError::raise(
          &stat_ref.base.base.location,
          format_args!("Exceeded return count limit; simplify the code to compile"),
        );
      }

      let _rs = self.reg_scope();
      let mut temp = 0u8;
      let mut consecutive = false;
      let mut mult_ret = false;

      if let Some((first_expr, rest)) = stat_ref.list.as_slice().split_first() {
        let reg = self.get_expr_local_reg(*first_expr);
        if reg >= 0 {
          temp = reg as u8;
          consecutive = true;
          for (i, &expr) in rest.iter().enumerate() {
            if self.get_expr_local_reg(expr) != (temp as i32 + (i + 1) as i32) {
              consecutive = false;
              break;
            }
          }
        }
      }

      if !consecutive && !stat_ref.list.is_empty() {
        temp = self.alloc_reg(&stat_ref.base.base, stat_ref.list.size as u32);
        for (i, &expr) in stat_ref.list.iter().enumerate() {
          if i + 1 == stat_ref.list.size {
            // Safety: list 元素为 parser 接线的存活表达式指针，借用只覆盖该次调用。
            mult_ret = self.compile_expr_temp_mult_ret(unsafe { &mut *expr }, temp + i as u8);
          } else {
            // Safety: 同上，compile_expr_temp_top 对该节点只读遍历。
            self.compile_expr_temp_top(unsafe { &mut *expr }, temp + i as u8);
          }
        }
      }

      self.close_locals(0);
      // cpp:4041-4042：本函数出现过 multret RETURN 后不可内联（LPF_INLINABLE 判定）
      if mult_ret {
        self.has_multi_ret = true;
      }
      self.bc_mut().emit_abc(
        LuauOpcode::LOP_RETURN,
        temp,
        if mult_ret {
          0
        } else {
          (stat_ref.list.size + 1) as u8
        },
        0,
      );
    }
  }

  /// `condition`/`body` 已句柄化（Node）：as_ptr 仅作既有裸指针 API 的桥接；
  /// 回跳 patch 依赖 `loop_jumps` 水位与当前编译帧一致。
  pub(crate) fn compile_stat_while(&mut self, stat_ref: &AstStatWhile) {
    // 优化：条件恒假 => 根本没有循环！
    if is_constant_false(&self.constants, stat_ref.condition.into()) {
      return;
    }

    let (_old_locals, old_jumps) = self.begin_loop();

    let loop_label = self.bc().emit_label();

    let else_jump = self.compile_condition_value(stat_ref.condition.as_ptr(), None, false);

    self.compile_stat(stat_ref.body.as_ptr().cast::<AstStat>());

    let cont_label = self.bc().emit_label();
    let back_label = self.bc().emit_label();

    // cpp `setDebugLine(stat->condition)`：JUMPBACK 的行号取条件表达式，
    // 取整个 while 节点会让行号落在 `while` 关键字行。
    // condition 已句柄化：`get` 交出的共享引用即存活证明，只读行号无需 unsafe。
    self.set_debug_line_ast_node(&stat_ref.condition.get().base);

    // 注：这里用 JUMPBACK 而非 JUMP，因 JUMPBACK 可中断——每个循环至少要有一条可中断指令
    self.bc_mut().emit_ad(LuauOpcode::LOP_JUMPBACK, 0, 0);

    let end_label = self.bc().emit_label();

    self.patch_jump(&stat_ref.base.base, back_label, loop_label);
    self.patch_jumps(&stat_ref.base.base, &else_jump, end_label);

    self.end_loop(&stat_ref.base.base, old_jumps, end_label, cont_label);
  }

  /// C++ `compileAssign`：把源寄存器写入左值（set 路径），纯转发到
  /// `compile_l_value_use`；`target_expr` 的 cpp null 哨兵已 Option 化。
  pub(crate) fn compile_assign(
    &mut self,
    lv: &LValue,
    source: u8,
    target_expr: Option<*mut AstExpr>,
  ) {
    self.compile_l_value_use(lv, source, true, target_expr);
  }

  /// 对应 cpp `Compiler::compileLValue`。原 `pub(crate) unsafe fn` 已 safe 化：
  /// `node` 由赋值左值链路传入（arena 存活 `AstExpr` 基指针，动态类型为
  /// Local/Global/IndexName/IndexExpr 之一，RTTI 未命中走断言兜底分支）；`rs` 为
  /// 包住本赋值的宿主 RegScope——index/value 临时寄存器在其中分配，返回的
  /// `LValue` 内寄存器编号以 `rs` 水位为界。只读分支全部走槽位门面；残余窄
  /// 独占借用仅 local 登记字段一处写穿。
  pub(crate) fn compile_l_value(&mut self, node: *mut AstExpr, rs: &mut RegScope) -> LValue {
    {
      // 门面解引用：`node` 是函数契约保证的合法 `AstExpr` 基指针（arena 分配、
      // 非空、地址稳定），取基类首字段与派生指针基址重合。
      let base = &ast_slot_ref(node)
        .expect("compile_l_value 契约：node 为 arena 存活 AstExpr 基指针")
        .base;

      self.set_debug_line_ast_node(base);

      // 门面判型+下转：命中即动态类型 AstExprLocal，且与 `node` 指向同一 arena 节点。
      if let Some(expr) = ast_slot_try_as::<AstExprLocal, _>(node) {
        // Safety: expr.local 已句柄化恒非空（解析器接线存活 AstLocal），本入口持共享
        // 借用（cpp 非 const 透传形态），&mut 重建经 as_ptr 裸出口；针对该独立节点，
        // 与 `rs`/`self` 可变借用不重叠，编译链对其只作登记字段读写。
        let local = unsafe { &mut *expr.local.as_ptr() };
        // cpp:3531 条件还含 `(!LuauOptimizeExportTable ||
        // !exports.exportedFunctions.contains(local))`——该旗标未移植
        // （exportedFunctions 恒空，见 exports_is_empty 台账），此处条件恒真
        if fflag::LuauExportValueSyntax.get() && local.is_exported {
          return LValue {
            kind: Kind::IndexName,
            reg: self.get_export_table_reg(base),
            name: sref_ast_name(local.name),
            location: base.location,
            ..Default::default()
          };
        }
        let reg = self.get_expr_local_reg(node);
        if reg >= 0 {
          LValue {
            kind: Kind::Local,
            reg: reg as u8,
            location: base.location,
            ..Default::default()
          }
        } else {
          LUAU_ASSERT!(expr.upvalue);
          LValue {
            kind: Kind::Upvalue,
            upval: self.get_upval(local),
            location: base.location,
            ..Default::default()
          }
        }
      // 门面判型+下转：命中即动态类型 AstExprGlobal 的共享借用，name 字段只读。
      } else if let Some(expr) = ast_slot_try_as::<AstExprGlobal, _>(node) {
        if DebugLuauUserDefinedClasses.get()
          && let Some(&class_local) = self.class_locals.find(&expr.name)
        {
          CompileError::raise(
            &expr.base.base.location,
            core::format_args!(
              "'{}' refers to a class and cannot be used as a variable name (defined on line {})",
              sref_ast_name(expr.name),
              // class_locals 登记项为 parser 接线的 arena 存活节点句柄。
              class_local.borrow().location.begin.line + 1
            ),
          );
        }

        LValue {
          kind: Kind::Global,
          name: sref_ast_name(expr.name),
          location: base.location,
          ..Default::default()
        }
      // 门面判型+下转：命中即动态类型 AstExprIndexName；expr 已句柄化恒非空，
      // index 子槽存活，compile_expr_auto 只读消费裸指针（经 as_ptr 桥接）。
      } else if let Some(expr) = ast_slot_try_as::<AstExprIndexName, _>(node) {
        LValue {
          kind: Kind::IndexName,
          reg: self.compile_expr_auto(expr.expr.as_ptr(), rs),
          name: sref_ast_name(expr.index),
          location: base.location,
          ..Default::default()
        }
      // 门面判型+下转：命中即动态类型 AstExprIndexExpr；expr.expr/index 已句柄化
      // 恒非空，既有裸指针编译 API 经 as_ptr 桥接（rs 为包住本赋值的宿主 RegScope）。
      } else if let Some(expr) = ast_slot_try_as::<AstExprIndexExpr, _>(node) {
        let reg = self.compile_expr_auto(expr.expr.as_ptr(), rs);
        self.compile_l_value_index(reg, expr.index.as_ptr(), rs)
      } else {
        LUAU_ASSERT!(false);
        LValue {
          kind: Kind::Local,
          location: base.location,
          ..Default::default()
        }
      }
    }
  }

  /// 对应 cpp `Compiler::compileLValueIndex`。原裸指针契约 fn 已 safe 化：
  /// `reg` 为当前窗口内已分配、持有被索引表的寄存器编号（SETTABLE/GETTABLE 的
  /// 基址槽）；`index` 为 parser 记录的存活 `AstExpr` 下标节点指针（只读消费经
  /// 槽位门面物化）；`rs` 为宿主编译作用域的 RegScope，本函数
  /// 在其内分配 key/value 临时寄存器。
  pub(crate) fn compile_l_value_index(
    &mut self,
    reg: u8,
    index: *mut AstExpr,
    rs: &mut RegScope,
  ) -> LValue {
    {
      // 门面无借用解引用：`index` 是 parser 记录的存活 `AstExpr` 下标节点指针
      // （函数契约：非空、arena 地址稳定），读取基类首字段全程只读。
      let base = &ast_slot_ref(index)
        .expect("index 为 parser 记录的存活 AstExpr 下标节点")
        .base;
      let cv = self.get_constant(index);
      match cv {
        // 整数下标 1..=256 走 IndexNumber 快路径
        Constant::Number(value_number)
          if (1.0..=256.0).contains(&value_number)
            && (value_number as i32) as f64 == value_number =>
        {
          LValue {
            kind: Kind::IndexNumber,
            reg,
            number: (value_number as i32 - 1) as u8,
            location: base.location,
            ..Default::default()
          }
        }
        Constant::Str(_) => LValue {
          kind: Kind::IndexName,
          reg,
          name: sref_ast_array_u8(cv.get_string()),
          location: base.location,
          ..Default::default()
        },
        _ => LValue {
          kind: Kind::IndexExpr,
          reg,
          index: self.compile_expr_auto(index, rs),
          location: base.location,
          ..Default::default()
        },
      }
    }
  }

  /// C++ `compileLValueUse`。`target_expr` 为赋值/取值目标的 AST 表达式指针，
  /// 缺省（原 null 哨兵）以 `None` 表达；仅用于 hint 分支的 RTTI 判型，不解引用。
  pub(crate) fn compile_l_value_use(
    &mut self,
    lv: &LValue,
    reg: u8,
    set: bool,
    target_expr: Option<*mut AstExpr>,
  ) {
    // C++ `compileLValueUse` 以 `setDebugLine(lv.location)` 开头，让 store/load
    // 指令归属下标自身的 location（如 `a["b"]["c"]["d"] = 4` 的 `["d"]` 行），
    // 而非编译 lvalue 基座时残留的行号。
    self.set_debug_line_location(&lv.location);
    match lv.kind {
      Kind::Local => {
        if set {
          self.bc_mut().emit_abc(LuauOpcode::LOP_MOVE, lv.reg, reg, 0);
        } else {
          self.bc_mut().emit_abc(LuauOpcode::LOP_MOVE, reg, lv.reg, 0);
        }
      }
      Kind::Upvalue => {
        if set {
          self
            .bc_mut()
            .emit_abc(LuauOpcode::LOP_SETUPVAL, reg, lv.upval, 0);
        } else {
          self
            .bc_mut()
            .emit_abc(LuauOpcode::LOP_GETUPVAL, reg, lv.upval, 0);
        }
      }
      // Global/IndexName 两臂同构（字符串常量键入表 + hash 寻址的 SET/GET 指令对，
      // 只差操作码与第二寄存器），坍缩为一条骨架；IndexName 额外携带 hint 分支。
      Kind::Global | Kind::IndexName => {
        let (set_op, get_op, idx_reg) = if lv.kind == Kind::Global {
          (LuauOpcode::LOP_SETGLOBAL, LuauOpcode::LOP_GETGLOBAL, 0u8)
        } else {
          (
            LuauOpcode::LOP_SETTABLEKS,
            LuauOpcode::LOP_GETTABLEKS,
            lv.reg,
          )
        };
        let cid = self.bc_mut().add_constant_string(lv.name.clone());
        self.check_constant(cid, &lv.location);

        let hash = bytecode_builder_get_string_hash(lv.name.clone()) as u8;
        self
          .bc_mut()
          .emit_abc(if set { set_op } else { get_op }, reg, idx_reg, hash);
        self.bc_mut().emit_aux(cid as u32);

        // 门面判型+下转：target_expr 为 Some(存活 AST 表达式指针) 或 None；判 null +
        // class index，命中即动态类型 AstExprIndexName，其 expr 子指针指向 arena
        // 存活节点，hint_* 全程只读。
        if lv.kind == Kind::IndexName
          && let Some(target_index_name) =
            target_expr.and_then(ast_slot_try_as::<AstExprIndexName, _>)
        {
          self.hint_table_reg(target_index_name.expr.as_ptr(), lv.reg, 2);
        }
      }
      Kind::IndexNumber => {
        if set {
          self
            .bc_mut()
            .emit_abc(LuauOpcode::LOP_SETTABLEN, reg, lv.reg, lv.number);
        } else {
          self
            .bc_mut()
            .emit_abc(LuauOpcode::LOP_GETTABLEN, reg, lv.reg, lv.number);
        }

        // Safety: target_expr 为 Some(存活 AST 表达式指针) 或 None；门面命中即动态类型
        // AstExprIndexExpr，expr 子指针存活，hint_* 只读。
        if let Some(target_index_expr) =
          target_expr.and_then(ast_slot_try_as::<AstExprIndexExpr, _>)
        {
          // expr 已句柄化；hint_table_reg 为既有裸指针 API，经 as_ptr 桥接。
          self.hint_table_reg(target_index_expr.expr.as_ptr(), lv.reg, 1);
        }
      }
      Kind::IndexExpr => {
        if set {
          self
            .bc_mut()
            .emit_abc(LuauOpcode::LOP_SETTABLE, reg, lv.reg, lv.index);
        } else {
          self
            .bc_mut()
            .emit_abc(LuauOpcode::LOP_GETTABLE, reg, lv.reg, lv.index);
        }

        // Safety: target_expr 为 Some(存活 AST 表达式指针) 或 None；门面命中即动态类型
        // AstExprIndexExpr，expr/index 子指针存活，hint_* 只读。
        if let Some(target_index_expr) =
          target_expr.and_then(ast_slot_try_as::<AstExprIndexExpr, _>)
        {
          // expr/index 已句柄化；hint_* 为既有裸指针 API，经 as_ptr 桥接。
          self.hint_table_reg(target_index_expr.expr.as_ptr(), lv.reg, 1);
          self.hint_number_reg(target_index_expr.index.as_ptr(), lv.index);
        }
      }
    }
  }

  /// 编译类声明语句，对应 cpp `compileClassDeclaration`
  /// （cpp/Compiler/src/Compiler.cpp:1709）：`decl` 为分发器判型后调用方持有的
  /// arena 存活 `AstStatClass`（`name` 由 parser 保证非空）；须在
  /// `DebugLuauUserDefinedClasses` 旗标开启时调用（入口断言）。方法体子指针
  /// 交 `compile_expr_function`，其 arena 存活契约沿链传递。
  pub(crate) fn compile_class_declaration(&mut self, decl: &mut AstStatClass) {
    LUAU_ASSERT!(fflag::DebugLuauUserDefinedClasses.get());

    // 类声明编译全程对 AST 只读，无其他写者。
    let decl_ref = decl;
    // Safety: 同 cpp 直接解引用 decl->name，类名指针由解析器保证存活非空。
    let name_ref = unsafe { &*decl_ref.name };
    let class_name = name_ref.name;

    let class_local = self.class_locals.find(&class_name).copied();
    let dest = match class_local.map(|l| self.get_local_reg(l)) {
      Some(r) if r >= 0 => r as u8,
      _ => {
        let d = self.alloc_reg(&decl_ref.base.base, 1);
        // Safety: decl_ref.name 为 parser 保证非空存活的类局部 AstLocal；
        // d 是 alloc_reg 刚返回的寄存器编号。
        self.push_local(unsafe { &*decl_ref.name }, d, u32::MAX);
        d
      }
    };

    if fflag::LuauExportValueSyntax.get() && decl_ref.exported {
      self.ensure_export_table(&decl_ref.base.base);
      // cpp `exportedClasses[decl->name] = dest`（Compiler.cpp:1732）：键为
      // AstLocal 指针——同名遮蔽的各 local 是独立键，不做按名去重
      self.exported_classes.push((decl_ref.name, dest));
    }

    let _rs = self.reg_scope();

    let instr_c_val: u8 = u8::from(decl_ref.open);
    if !decl_ref.super_.is_null() {
      let super_reg = self.get_expr_local_reg(decl_ref.super_);
      if super_reg >= 0 {
        self
          .bc_mut()
          .emit_abc(LuauOpcode::LOP_NEWCLASS, dest, super_reg as u8, instr_c_val);
      } else {
        let super_dest = self.alloc_reg(&decl_ref.base.base, 1);
        // Safety: 上方 !is_null 短路已确认 super_ 非空，其指向 arena 存活表达式；
        // super_dest 为 alloc_reg 分配寄存器。
        self.compile_expr(unsafe { &mut *decl_ref.super_ }, super_dest, false);
        self
          .bc_mut()
          .emit_abc(LuauOpcode::LOP_NEWCLASS, dest, super_dest, instr_c_val);
      }
    } else {
      self.bc_mut().emit_abc(
        LuauOpcode::LOP_NEWCLASS,
        dest,
        INVALID_SUPER_REG,
        instr_c_val,
      );
    }

    let aux_offset = self.bc().emit_label();
    self.bc_mut().emit_aux(DUMMY_AUX);

    let class_name_cid = self.bc_mut().add_constant_string(sref_ast_name(class_name));
    self.check_constant(class_name_cid, &name_ref.location);

    let mut shape = ClassShape {
      class_name: class_name_cid,
      ..Default::default()
    };

    let temp = self.alloc_reg(&decl_ref.base.base, 1);
    let mut has_explicit_constructor = false;

    for member in decl_ref.members.as_slice() {
      match member {
        Variant2::V0(prop) => {
          let cid = self.bc_mut().add_constant_string(sref_ast_name(prop.name));
          self.check_constant(cid, &prop.name_location);
          shape.property_names.push(cid);
        }
        Variant2::V1(method) => {
          // method.function 为 parser 接线、本次编译结束前 arena 存活的 AstExprFunction；
          // temp 是本函数 alloc_reg 出的临时槽。
          self.compile_expr_function(method.function, temp);
          let cid = self
            .bc_mut()
            .add_constant_string(sref_ast_name(method.function_name));
          // Safety: method.function 同上存活，读 location 仅供报错定位。
          self.check_constant(cid, unsafe { &(*method.function).base.base.location });
          shape.method_names.push(cid);
          self.emit_abc_aux(LuauOpcode::LOP_NEWCLASSMEMBER, dest, 0, temp, cid as u32);
          if method.function_name.as_str() == Some(INIT_NAME) {
            has_explicit_constructor = true;
          }
        }
      }
    }

    // 类默认具备 new 与 __init 方法
    let new_cid = self.bc_mut().add_constant_string(StringRef::from(NEW_NAME));
    self.check_constant(new_cid, &decl_ref.base.base.location);
    shape.method_names.push(new_cid);

    if !has_explicit_constructor {
      let init_cid = self
        .bc_mut()
        .add_constant_string(StringRef::from(INIT_NAME));
      self.check_constant(init_cid, &decl_ref.base.base.location);
      shape.method_names.push(init_cid);
    }

    let class_const = self.bc_mut().add_class_shape(shape);
    self.check_constant(class_const, &decl_ref.base.base.location);
    self.bc_mut().patch_aux(aux_offset, class_const);
  }

  /// 循环入口登记（cpp while/repeat/for/for-in 四个循环函数的同构入口样板）：
  /// 记录局部栈与跳转表水位、压入 Loop 帧并置 `has_loops`。返回
  /// `(old_locals, old_jumps)` 水位对，供循环体收尾回卷与 `end_loop` 回填。
  pub(crate) fn begin_loop(&mut self) -> (usize, usize) {
    let old_locals = self.local_stack.len();
    let old_jumps = self.loop_jumps.len();

    self.loops.push(Loop {
      local_offset: old_locals,
      local_offset_continue: old_locals,
      continue_used: None,
    });
    self.has_loops = true;

    (old_locals, old_jumps)
  }

  /// 循环收尾三件套（cpp 四个循环函数出口的 `patchLoopJumps`、`loopJumps.resize`
  /// 与 `loops.pop_back` 样板）：回填 break/continue 跳转、把 `loop_jumps` 水位
  /// 回卷（resize 填充值与 cpp 同源、从不被读取）并弹出 Loop 帧。
  ///
  /// `node` 仅供回填的报错路径读取 location。
  pub(crate) fn end_loop(
    &mut self,
    node: &AstNode,
    old_jumps: usize,
    end_label: usize,
    cont_label: usize,
  ) {
    self.patch_loop_jumps(node, old_jumps, end_label, cont_label);
    self.loop_jumps.resize(
      old_jumps,
      LoopJump {
        r#type: Type::Break,
        label: 0,
      },
    );
    self.loops.pop();
  }

  /// cpp Compiler.cpp `patchJump`：回填跳转距离；`node` 仅用于越界时定位报错。
  pub fn patch_jump(&mut self, node: &AstNode, label: usize, target: usize) {
    let ok = self.bc_mut().patch_jump_d(label, target);
    if !ok {
      CompileError::raise(
        &node.location,
        format_args!("{ERR_EXCEEDED_JUMP_DISTANCE_LIMIT}"),
      );
    }
  }

  /// cpp Compiler.cpp `patchJumps`：将一批跳转标签统一回填到同一目标。
  /// 标签数组只读遍历（cpp 的引用形参在 Rust 侧收敛为 `&[usize]`）。
  pub(crate) fn patch_jumps(&mut self, node: &AstNode, labels: &[usize], target: usize) {
    for &l in labels {
      self.patch_jump(node, l, target);
    }
  }

  /// cpp Compiler.cpp `patchLoopJumps`：回填 break/continue 跳转。
  pub(crate) fn patch_loop_jumps(
    &mut self,
    node: &AstNode,
    old_jumps: usize,
    end_label: usize,
    cont_label: usize,
  ) {
    LUAU_ASSERT!(old_jumps <= self.loop_jumps.len());

    for i in old_jumps..self.loop_jumps.len() {
      let (kind, label) = {
        let lj: &LoopJump = &self.loop_jumps[i];
        (lj.r#type, lj.label)
      };

      match kind {
        Type::Break => self.patch_jump(node, label, end_label),
        Type::Continue => self.patch_jump(node, label, cont_label),
      }
    }
  }

  /// C++ `isStatBreak`：单语句块中的 break，或裸 break。
  pub(crate) fn is_stat_break(&mut self, node: *mut AstStat) -> bool {
    // 门面判型+下转：null 输入或不匹配返回 None；命中即 node 确为 AstStatBlock
    // （repr(C) 首字段一致）；node 是 parser arena 中编译期只读存活的语句指针。
    if let Some(block) = ast_slot_try_as::<AstStatBlock, _>(node) {
      // len == 1 守卫下界内取 body 首槽句柄，判型走门面（null 折叠 false）
      block.body.len() == 1 && ast_slot_is::<AstStatBreak, _>(block.body.at(0).as_ptr())
    } else {
      ast_slot_is::<AstStatBreak, _>(node)
    }
  }

  /// C++ `extractStatContinue`：单语句块的 continue 语句节点，否则 `None`。
  /// 返回 [`Node<AstStatContinue>`] 仅作后续只读定位的地址句柄。
  pub(crate) fn extract_stat_continue(
    &mut self,
    block: &AstStatBlock,
  ) -> Option<Node<AstStatContinue>> {
    let body = &block.body;
    if body.len() != 1 {
      return None;
    }
    // len==1 守卫下界内取首槽（非空由 Node 构造端证明），
    // 门面判型+下转：stat 为 parser 块体首元素（block 借用证明内存活），
    // class index 校验后 Some 结果确为 AstStatContinue。
    ast_slot_try_as::<AstStatContinue, _>(body.at(0).as_ptr()).map(Node::from_ref)
  }

  /// 校验 `continue` 跳过的 local 随后未被 `until` 条件使用；`start` 为
  /// continue 语句在循环体内的下标。
  pub(crate) fn validate_continue_until(
    &mut self,
    cont: &AstStat,
    condition: &mut AstExpr,
    body: &AstStatBlock,
    start: usize,
  ) {
    let mut visitor = UndefinedLocalVisitor {
      compiler: self,
      undef: None,
      locals: DenseHashSet::default(),
    };

    // Safety: body.body 元素为 parser 接线的存活语句指针（body 借用即存活证明），
    // ast_node_try_as_ptr 经 RTTI 校验，Some 命中即动态类型正确、只读取 vars/name 键。
    unsafe {
      for &stat in body.body.iter_nodes().skip(start) {
        if let Some(local_stat) = ast_node_try_as_ptr::<AstStatLocal>(stat) {
          for &var in local_stat.vars.iter() {
            visitor.locals.insert(var.into());
          }
        } else if let Some(func_stat) = ast_node_try_as_ptr::<AstStatLocalFunction>(stat) {
          visitor.locals.insert(func_stat.name.into());
        }
      }
    }

    // Safety: ast_expr_visit 按 cpp 非 const visit 语义对子树物化 &mut；condition
    // 子树在本调用内被独占借用，visitor 只写自身 locals/undef 集合、不写 AST。
    unsafe { ast_expr_visit(from_mut(condition), &mut visitor) };

    if let Some(undef) = visitor.undef {
      // undef 由 visitor 从子树收集的存活句柄（契约见 `Node::borrow`）。
      let undef_name = undef.borrow().name;
      CompileError::raise(
        &condition.base.location,
        format_args!(
          "Local {} used in the repeat..until condition is undefined because continue statement on line {} jumps over it",
          undef_name,
          cont.base.location.begin.line + 1
        ),
      );
    }
  }

  /// 由 `compile_stat_for` 在 optimization_level≥2 且 from/to 为常量时调用；
  /// 阈值参数仅数值比较。返回 true 表示已按展开序列完成编译。
  pub(crate) fn try_compile_unrolled_for(
    &mut self,
    stat_ref: &AstStatFor,
    threshold_base: i32,
    threshold_max_boost: i32,
  ) -> bool {
    let one = cnum(1.0);

    let fromc = self.get_constant(stat_ref.from);
    let toc = self.get_constant(stat_ref.to);
    // step 已落可空 OptNode：is_some/is_none 承接 cpp `step != nullptr`，
    // as_ptr 桥交指针形态的 get_constant。
    let stepc = if stat_ref.step.is_some() {
      self.get_constant(stat_ref.step.as_ptr())
    } else {
      one
    };

    // 三者均为 Number 常量时才可折叠；同时缓存数值供展开编译复用
    let mut nums = (0.0, 0.0, 0.0);
    let trip_count = match (&fromc, &toc, &stepc) {
      (Constant::Number(f), Constant::Number(t), Constant::Number(s)) => {
        nums = (*f, *t, *s);
        get_trip_count(*f, *t, *s)
      }
      _ => -1,
    };

    if trip_count < 0 {
      return self.reject_with_remark(format_args!("loop unroll failed: invalid iteration count"));
    }

    if trip_count > threshold_base {
      return self.reject_with_remark(format_args!(
        "loop unroll failed: too many iterations ({})",
        trip_count
      ));
    }

    if let Some(lv) = self.variables.find(&stat_ref.var.into())
      && lv.written
    {
      return self.reject_with_remark(format_args!("loop unroll failed: mutable loop variable"));
    }

    let var = Node::from(stat_ref.var);
    // Safety: body 为 parser 接线、stat_ref 借用期内存活的循环块，
    // &mut 仅作 model_cost 只读遍历入口。
    let cost_model = model_cost(
      unsafe { &mut *stat_ref.body.as_ptr().cast::<AstNode>() },
      from_ref(&var),
      // cpp Compiler.cpp:4167 传 `builtins` 成员（O>=1 恒有数据），
      // 而非 O2 才赋值的 builtinsFold 别名指针
      &self.builtins,
      &self.constants,
    );

    let unrolled_cost = compute_cost(cost_model, &[true]) * trip_count;
    let baseline_cost = (compute_cost(cost_model, &[]) + 1) * trip_count;
    let unroll_profit = if unrolled_cost == 0 {
      threshold_max_boost
    } else {
      threshold_max_boost.min(K_COST_PERCENT_SCALE * baseline_cost / unrolled_cost)
    };

    let threshold = threshold_base * unroll_profit / K_COST_PERCENT_SCALE;

    if unrolled_cost > threshold {
      self.bc_mut().add_debug_remark(format_args!(
        "loop unroll failed: too expensive (iterations {}, cost {}, profit {:.2}x)",
        trip_count,
        unrolled_cost,
        unroll_profit as f64 / K_COST_PERCENT_SCALE as f64
      ));
      return false;
    }

    self.bc_mut().add_debug_remark(format_args!(
      "loop unroll succeeded (iterations {}, cost {}, profit {:.2}x)",
      trip_count,
      unrolled_cost,
      unroll_profit as f64 / K_COST_PERCENT_SCALE as f64
    ));

    // stat_ref 即入口存活借用，直接交给已降 safe 的展开编译。
    self.compile_unrolled_for(stat_ref, trip_count, nums.0, nums.2);
    true
  }

  /// `trip_count`/`from`/`step` 须是 `stat` 常量边界的求值结果（trip_count≥0
  /// 且与 `stat` 的 from/to/step 一致），展开序列按它们发射寄存器写入区间。
  pub(crate) fn compile_unrolled_for(
    &mut self,
    stat_ref: &AstStatFor,
    trip_count: i32,
    from: f64,
    step: f64,
  ) {
    let old_locals = self.local_stack.len();
    let old_jumps = self.loop_jumps.len();

    self.loops.push(Loop {
      local_offset: old_locals,
      local_offset_continue: old_locals,
      continue_used: None,
    });

    self.expr_changes.clear();
    self.local_changes.clear();

    for iv in 0..trip_count {
      *self.locstants.get_or_insert(stat_ref.var.into()) =
        Constant::Number(from + f64::from(iv) * step);

      // Safety: body 句柄出自 parser 存活契约（本函数按 & 借用消费，as_ptr 桥回
      // 裸指针重建 &mut 与 cpp 直传 stat->body 同语义），&mut 借用仅覆盖折叠遍历
      // 入口，折叠对 AST 只读（只写 map 与名字表）。
      self.fold_constants(
        unsafe { &mut *stat_ref.body.as_ptr().cast::<AstNode>() },
        iv == 0,
      );

      let iter_jumps = self.loop_jumps.len();
      self.compile_stat(stat_ref.body.as_ptr().cast::<AstStat>());

      let cont_label = self.bc().emit_label();

      for i in iter_jumps..self.loop_jumps.len() {
        if self.loop_jumps[i].r#type == LoopJumpType::Continue {
          self.patch_jump(&stat_ref.base.base, self.loop_jumps[i].label, cont_label);
        }
      }
    }

    let end_label = self.bc().emit_label();

    for i in old_jumps..self.loop_jumps.len() {
      if self.loop_jumps[i].r#type == LoopJumpType::Break {
        self.patch_jump(&stat_ref.base.base, self.loop_jumps[i].label, end_label);
      }
    }

    self.loop_jumps.resize(
      old_jumps,
      LoopJump {
        r#type: LoopJumpType::Break,
        label: 0,
      },
    );

    self.loops.pop();

    *self.locstants.get_or_insert(stat_ref.var.into()) = Constant::Unknown;

    undo_changes_expr(&mut self.constants, &self.expr_changes);
    undo_changes_local(&mut self.locstants, &self.local_changes);
  }
}

const INIT_NAME: &str = "__init";
const NEW_NAME: &str = "new";
const DUMMY_AUX: u32 = 0xDEAD_BEEF;
const INVALID_SUPER_REG: u8 = 0xFF;
