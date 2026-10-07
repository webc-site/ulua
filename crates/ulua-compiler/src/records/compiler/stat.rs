//! `Compiler` 语句编译与循环控制流：`compile_stat` 分发、局部/赋值/函数声明语句、
//! 循环边界与跳转补丁（对照 cpp `Compiler.cpp` 的 compileStat* / beginLoop 段）。
use core::{cmp::max, slice::from_ref};

use ulua_ast::{
  enums::ast_stat_ref::AstStatRef,
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
    ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn,
    ast_stat_if::AstStatIf,
    ast_stat_local::AstStatLocal,
    ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn,
    ast_stat_while::AstStatWhile,
  },
  visit::ast_expr_visit_ref,
};
use ulua_bytecode::{
  methods::bytecode_builder_get_string_hash::bytecode_builder_get_string_hash,
  records::{class_shape::ClassShape, string_ref::StringRef},
};
use ulua_common::{
  enums::{luau_bytecode_type::LuauBytecodeType, luau_opcode::LuauOpcode},
  fflag,
  fflag::{DebugLuauUserDefinedClasses, LuauCompileLoopUnrollZero},
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
    ast_slot_ref::{ast_slot_ref, ast_slot_try_as},
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
      K_MAX_K_CONST_INDEX,
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

impl Compiler {
  /// C++ `compileStat`：按动态类型分发到各语句编译器。
  ///
  /// 入参是节点身份句柄（review §2：`*mut AstStat` → 句柄）：借用只在
  /// [`Node::get`] 一处物化，半径由本函数持有的 `node` 局部决定。
  pub(crate) fn compile_stat(&mut self, node: Node<AstStat>) {
    let stat_node = node.get();
    let base = &stat_node.base;
    self.set_debug_line_ast_node(base);
    if self.options.coverage_level >= 1 && self.needs_coverage(base) {
      self.bc_mut().emit_abc(LuauOpcode::LOP_COVERAGE, 0, 0, 0);
    }

    match stat_node.as_stat_ref() {
      AstStatRef::Block(stat) => {
        let _rs = self.reg_scope();
        let old_locals = self.local_stack.len();
        if fflag::LuauExportValueSyntax.get() {
          self.block_depth += 1;
        }
        for body_stat in stat.body.iter_nodes() {
          let body_stat = Node::from(*body_stat);
          self.compile_stat(body_stat);
          if always_terminates(&self.constants, body_stat) {
            break;
          }
        }
        if fflag::LuauExportValueSyntax.get() {
          self.block_depth -= 1;
        }
        self.close_locals(old_locals);
        self.pop_locals(old_locals);
      }
      AstStatRef::If(stat) => self.compile_stat_if(stat),
      AstStatRef::While(stat) => self.compile_stat_while(stat),
      AstStatRef::Repeat(stat) => self.compile_stat_repeat(stat),
      AstStatRef::Break(_) => {
        LUAU_ASSERT!(!self.loops.is_empty());
        self.close_locals(self.loops.last().unwrap().local_offset);
        let label = self.bc().emit_label();
        self.bc_mut().emit_ad(LuauOpcode::LOP_JUMP, 0, 0);
        self.loop_jumps.push(LoopJump {
          r#type: Break,
          label,
        });
      }
      AstStatRef::Continue(stat) => {
        LUAU_ASSERT!(!self.loops.is_empty());
        // 栈顶一次可变借用即完成「未设置才置位」，免三次 `last()` 链查找
        self
          .loops
          .last_mut()
          .unwrap()
          .continue_used
          .get_or_insert(Node::from_ref(stat));
        let local_offset_continue = self.loops.last().unwrap().local_offset_continue;
        self.close_locals(local_offset_continue);
        let label = self.bc().emit_label();
        self.bc_mut().emit_ad(LuauOpcode::LOP_JUMP, 0, 0);
        self.loop_jumps.push(LoopJump {
          r#type: Continue,
          label,
        });
      }
      AstStatRef::Return(stat) => {
        if self.options.optimization_level >= 2 && !self.inline_frames.is_empty() {
          self.compile_inline_return(stat);
        } else {
          self.compile_stat_return(stat);
        }
      }
      AstStatRef::Expr(stat) => {
        // expr 已句柄化（node_handle::Node）：判型下转挂在句柄上，命中即交出
        // 独占借用；只读臂经 `get` 直取共享借用（arena 存活前提由 Node 供给）。
        let mut expr = Node::from(stat.expr);
        if let Some(call) = expr.try_as_mut::<AstExprCall>() {
          self.compile_expr_call(call, self.reg_top as u8, 0, false, false);
        } else {
          self.compile_expr_side(expr.get());
        }
      }
      AstStatRef::Local(stat) => {
        if fflag::LuauExportValueSyntax.get() {
          for &var in stat.vars.iter() {
            if let Some(local) = Node::try_new(var) {
              self.check_exported_local(local.get(), &base.location);
            }
          }
        }
        self.compile_stat_local(stat);
      }
      AstStatRef::For(stat) => self.compile_stat_for(stat),
      AstStatRef::ForIn(stat) => self.compile_stat_for_in(stat),
      AstStatRef::Assign(stat) => self.compile_stat_assign(stat),
      AstStatRef::CompoundAssign(stat) => self.compile_stat_compound_assign(stat),
      AstStatRef::Function(stat) => self.compile_stat_function(stat),
      AstStatRef::LocalFunction(stat) => {
        let name = stat.name.get();
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
      AstStatRef::DeclareClass(stat) => {
        if fflag::DebugLuauUserDefinedClasses.get() {
          self.compile_class_declaration(stat);
        }
      }
      AstStatRef::TypeAlias(_)
      | AstStatRef::TypeFunction(_)
      | AstStatRef::DeclareFunction(_)
      | AstStatRef::DeclareGlobal(_)
      | AstStatRef::DeclareExternType(_)
      | AstStatRef::Error(_) => {}
    }
  }

  /// 对应 cpp `Compiler::compileStatAssign`。`stat_ref` 由 `compile_stat` 分发器
  /// 判型后共享借出（arena 存活、非空、编译期地址稳定）；`vars`/`values` 的
  /// `AstArray` 槽位受各自 `size` 界约束，取用即升为 [`Node`] 句柄，业务侧不再
  /// 出现裸 `*mut AstExpr`。只读遍历全部经句柄借用；残余窄独占借用仅 vars/values
  /// 表达式节点的编译期临时字段写穿。
  pub(crate) fn compile_stat_assign(&mut self, stat_ref: &AstStatAssign) {
    let mut rs = self.reg_scope();

    if stat_ref.vars.size == 1 && stat_ref.values.size == 1 {
      // 双 size==1 守卫下首槽经 as_slice 界内取回
      let var_expr = Node::from(stat_ref.vars.as_slice()[0]);
      let mut value_expr = Node::from(stat_ref.values.as_slice()[0]);
      let var = self.compile_l_value(var_expr, &mut rs);
      if var.kind == Kind::Local {
        self.compile_expr(value_expr.get_mut(), var.reg, false);
      } else {
        let reg = self.compile_expr_auto(value_expr.get(), &mut rs);
        self.set_debug_line_ast_node(&var_expr.get().base);
        self.compile_assign(&var, reg, Some(var_expr));
      }
      return;
    }

    let mut vars = Vec::with_capacity(stat_ref.vars.size);
    for &var_expr in stat_ref.vars.iter() {
      vars.push(Assignment {
        lvalue: self.compile_l_value(Node::from(var_expr), &mut rs),
        conflict_reg: K_INVALID_REG,
        value_reg: K_INVALID_REG,
      });
    }

    // stat_ref 为契约保证的存活 AstStatAssign；vars/values 切片受 size 界约束。
    self.resolve_assign_conflicts(&stat_ref.base, &mut vars, &stat_ref.values);

    // take 取较短一侧，等价于 C++ 的 `min(vars.size, values.size)` 循环
    for (i, value) in stat_ref.values.iter().take(vars.len()).enumerate() {
      let mut value = Node::from(*value);
      if i + 1 == stat_ref.values.size && stat_ref.vars.size > stat_ref.values.size {
        let rest = (stat_ref.vars.size - stat_ref.values.size + 1) as u32;
        let temp = self.alloc_reg(&stat_ref.base.base, rest);
        self.compile_expr_temp_n(value.get_mut(), temp, rest as u8, true);
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
          self.compile_expr(value.get_mut(), var.value_reg, false);
        } else {
          var.value_reg = self.compile_expr_auto(value.get(), &mut rs);
        }
      }
    }

    for &value in stat_ref.values.iter().skip(stat_ref.vars.size) {
      self.compile_expr_side(Node::from(value).get());
    }

    for (i, var) in vars.iter().enumerate() {
      LUAU_ASSERT!(var.value_reg != K_INVALID_REG);
      if var.lvalue.kind != Kind::Local {
        self.set_debug_line_location(&var.lvalue.location);
        // 越界=左值缺失（values 多于 vars 的 cpp null 补位）→ 切片 get()/Option
        let target_expr = stat_ref.vars.as_slice().get(i).copied().map(Node::from);
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

  /// 对应 cpp `Compiler::compileStatCompoundAssign`。`stat_ref` 由分发器判型后
  /// arena 存活借出（非空、地址稳定）；`var`/`value` 已是
  /// [`ulua_ast::records::node_handle::Node`]（非空证明内建），升为本 crate 的
  /// [`Node`] 句柄后交 `compile_l_value` / hint 门面（其 RTTI 分支要求 var 为
  /// l-value 形状：Local/IndexName/IndexExpr，由 parser 接线保证）。
  /// 残余窄独占借用仅为 Concat 实参节点的临时字段写穿。
  pub(crate) fn compile_stat_compound_assign(&mut self, stat_ref: &AstStatCompoundAssign) {
    let mut rs = self.reg_scope();
    // var 以地址句柄贯穿 compile_l_value / compile_l_value_use / compile_assign，
    // 全程不再出现裸指针；rs 为宿主 RegScope。
    let var_node = Node::from(stat_ref.var);
    let var = self.compile_l_value(var_node, &mut rs);
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
          self.compile_l_value_use(&var, target, false, Some(var_node));
        }
        // value 已句柄化：`.get()` 直供只读常量折叠。
        let rc = self.get_constant_number(stat_ref.value.get());
        if let Some(rc) = rc
          && (0..K_MAX_K_CONST_INDEX).contains(&rc)
        {
          let op = self.get_binary_op_arith(stat_ref.op, true);
          self.bc_mut().emit_abc(op, target, target, rc as u8);
        } else {
          let rr = self.compile_expr_auto(stat_ref.value.get(), &mut rs);
          let op = self.get_binary_op_arith(stat_ref.op, false);
          self.bc_mut().emit_abc(op, target, target, rr);
          if var.kind != Kind::Local {
            self.hint_temporary_reg_type(
              stat_ref.var,
              target as i32,
              LuauBytecodeType::LBC_TYPE_NUMBER,
              1,
            );
          }
          self.hint_number_reg(stat_ref.value, rr);
        }
      }
      AstExprBinaryOp::Concat => {
        // args 行走链以 arena 身份句柄承载（unroll_concats 只收集同树节点）。
        let mut args = vec![stat_ref.value.into()];
        self.unroll_concats(&mut args);
        let regs = self.alloc_reg(&stat_ref.base.base, (1 + args.len()) as u32);
        self.compile_l_value_use(&var, regs, false, Some(var_node));
        for (i, mut arg) in args.iter().copied().enumerate() {
          // arg 为同树存活节点句柄，&mut 借用经 Node 契约交求值路径（写穿限于
          // 该节点编译期临时字段）。
          if fflag::LuauCompileConcatTargetTop.get() {
            self.compile_expr_temp_top(arg.get_mut(), regs + 1 + i as u8);
          } else {
            self.compile_expr(arg.get_mut(), regs + 1 + i as u8, true);
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
      self.compile_assign(&var, target, Some(var_node));
    }
  }

  /// `stat_ref` 由分发器共享借出证明存活；`from`/`to`/`var`/`body`
  /// 已句柄化为 Node（parser 非空由类型层承载），`step` 落可空 OptNode 显式
  /// 判空后才解借用；`var` 交 `push_local` 登记，须长寿于本次编译。
  pub(crate) fn compile_stat_for(&mut self, stat_ref: &AstStatFor) {
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

    self.compile_expr(Node::from(stat_ref.from).get_mut(), regs + 2, true);
    self.compile_expr(Node::from(stat_ref.to).get_mut(), regs, true);

    if let Some(step) = stat_ref.step.to_option() {
      self.compile_expr(Node::from(step).get_mut(), regs + 1, true);
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
    // body 已句柄化（非空由类型层承载），向上转基类句柄交分发器。
    self.compile_stat(Node::from(stat_ref.body).cast::<AstStat>());

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

    // body 已句柄化（非空由类型层承载），向上转基类句柄交分发器。
    self.compile_stat(Node::from(stat_ref.body).cast::<AstStat>());

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

  /// `thenbody`/`elsebody` 已句柄化：非空由 `Node` 类型端兑现，`elsebody` 以
  /// `Option<Node<AstStat>>` 表达（`None` 即「无 else」），全程只读。
  pub(crate) fn compile_stat_if(&mut self, stat: &AstStatIf) {
    // thenbody 由 Node 类型端兑现非空，`get` 交出只读引用供 continue 抽取。
    let then_ref = stat.thenbody.get();
    let then_node = Node::from(stat.thenbody).cast::<AstStat>();
    let else_node = stat.elsebody.map(|n| Node::from_ref(n).cast::<AstStat>());
    let cond_node = Node::from(stat.condition);
    let cond_ref = stat.condition.get();

    if is_constant_false(&self.constants, stat.condition.into()) {
      if let Some(else_node) = else_node {
        self.compile_stat(else_node);
      }
      return;
    }

    // 句柄判型：condition 命中 AstExprBinary 才继续（arena 存活由句柄承载）。
    if let Some(cand) = cond_node.try_as::<AstExprBinary>()
      && cand.op == AstExprBinaryOp::And
      && is_constant_false(&self.constants, cand.right.into())
    {
      // left 已句柄化，`.get()` 直供只读入参。
      self.compile_expr_side(cand.left.get());
      if let Some(else_node) = else_node {
        self.compile_stat(else_node);
      }
      return;
    }

    if else_node.is_none()
      && self.is_stat_break(then_node)
      && !self.are_locals_captured(self.loops.last().unwrap().local_offset)
    {
      let else_jump = self.compile_condition_value(cond_ref, None, true);
      self.push_loop_jumps(Type::Break, else_jump);
      return;
    }

    let continue_statement = self.extract_stat_continue(then_ref);
    if else_node.is_none()
      && continue_statement.is_some()
      && !self.are_locals_captured(self.loops.last().unwrap().local_offset_continue)
    {
      if self.loops.last().unwrap().continue_used.is_none() {
        self.loops.last_mut().unwrap().continue_used = continue_statement;
      }
      let else_jump = self.compile_condition_value(cond_ref, None, true);
      self.push_loop_jumps(Type::Continue, else_jump);
      return;
    }

    let else_jump = self.compile_condition_value(cond_ref, None, false);
    self.compile_stat(then_node);

    if let Some(else_node) = else_node
      && !else_jump.is_empty()
    {
      if always_terminates(&self.constants, then_node) {
        let else_label = self.bc().emit_label();
        self.compile_stat(else_node);
        self.patch_jumps(&stat.base.base, &else_jump, else_label);
      } else {
        let then_label = self.bc().emit_label();
        self.bc_mut().emit_ad(LuauOpcode::LOP_JUMP, 0, 0);
        let else_label = self.bc().emit_label();
        self.compile_stat(else_node);
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
      let var_node = Node::from(stat_ref.vars.as_slice()[0]);

      // re_node 先绑为局部句柄：借用只能在 if 体内有效（返回引用不可穿越 map）。
      let re_node = self.get_expr_local(value);
      if let Some(re_node) = re_node {
        let re = re_node.get();
        // cpp `re->local` 槽位已句柄化恒非空（旧「空槽折叠」分支类型端不可达）。
        let rv_slot = re.local;
        let lv_written = self.variables.find(&var_node).is_some_and(|lv| lv.written);
        let rv_written = self
          .variables
          .find(&rv_slot.into())
          .is_some_and(|rv| rv.written);
        let reg = self.get_expr_local_reg(value);
        // rv_slot 已句柄化恒非空：.get() 安全借用读 is_exported。
        let exported_free = !var_node.get().is_exported && !rv_slot.get().is_exported;

        if let Some(reg) = reg
          && !lv_written
          && !rv_written
          && exported_free
        {
          let allocpc = self.bc().get_debug_pc();
          self.push_local(var_node.get(), reg, allocpc);
          return;
        }
      }
    }

    let vars = self.alloc_reg(&stat_ref.base.base, stat_ref.vars.size as u32);
    let allocpc = self.bc().get_debug_pc();

    self.compile_expr_list_temp(&stat_ref.values, vars, stat_ref.vars.size as u8, true);

    for (i, local) in stat_ref.vars.iter_nodes().enumerate() {
      // 槽位为 parser 接线的存活 AstLocal（cpp 处直接解引用）：`iter_nodes` 只读
      // 遍历把 arena 存活契约压在引用上，非空由解引用点的一次性证明承载。
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
  /// 为 Node（parser 非空由类型层承载），`get` 直出借用，as_ptr 仅作
  /// 既有裸指针 API 的桥接；循环回跳 patch 依赖 `loop_jumps`/`local_stack` 水位
  /// 与 self 当前编译帧一致。
  pub(crate) fn compile_stat_repeat(&mut self, stat_ref: &AstStatRepeat) {
    let body = stat_ref.body.get();

    let (old_locals, old_jumps) = self.begin_loop();

    let loop_label = self.bc().emit_label();

    // cpp `RegScope rs(this)`：裸构造等价，但统一走构造函数便于维护
    let _rs = self.reg_scope();

    let mut continue_validated = false;
    let mut condition_locals = 0;

    for (i, body_stat) in body.body.iter_nodes().enumerate() {
      self.compile_stat(Node::from(*body_stat));

      self.loops.last_mut().unwrap().local_offset_continue = self.local_stack.len();

      if let Some(continue_used) = self.loops.last().unwrap().continue_used
        && !continue_validated
      {
        self.validate_continue_until(
          continue_used.cast::<AstStat>().get(),
          Node::from(stat_ref.condition),
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
      let skip_jump = self.compile_condition_value(stat_ref.condition.get(), None, true);

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

      if let Some((first_expr, rest)) = stat_ref.list.as_slice().split_first()
        && let Some(reg) = self.get_expr_local_reg(*first_expr)
      {
        temp = reg;
        consecutive = true;
        for (i, &expr) in rest.iter().enumerate() {
          let Ok(expected) = u8::try_from(temp as usize + i + 1) else {
            consecutive = false;
            break;
          };
          if self.get_expr_local_reg(expr) != Some(expected) {
            consecutive = false;
            break;
          }
        }
      }

      if !consecutive && !stat_ref.list.is_empty() {
        temp = self.alloc_reg(&stat_ref.base.base, stat_ref.list.size as u32);
        for (i, &expr) in stat_ref.list.iter().enumerate() {
          if i + 1 == stat_ref.list.size {
            mult_ret = self.compile_expr_temp_mult_ret(Node::from(expr).get_mut(), temp + i as u8);
          } else {
            self.compile_expr_temp_top(Node::from(expr).get_mut(), temp + i as u8);
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

    let else_jump = self.compile_condition_value(stat_ref.condition.get(), None, false);

    self.compile_stat(Node::from(stat_ref.body).cast::<AstStat>());

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
    target_expr: Option<Node<AstExpr>>,
  ) {
    self.compile_l_value_use(lv, source, true, target_expr);
  }

  /// 对应 cpp `Compiler::compileLValue`。原 `pub(crate) unsafe fn` 已 safe 化：
  /// `node` 由赋值左值链路交付的 [`Node`] 句柄承载（非空证明内建、arena 存活、
  /// 地址稳定），动态类型为 Local/Global/IndexName/IndexExpr 之一，RTTI 未命中走
  /// 断言兜底分支；`rs` 为包住本赋值的宿主 RegScope——index/value 临时寄存器在
  /// 其中分配，返回的 `LValue` 内寄存器编号以 `rs` 水位为界。
  pub(crate) fn compile_l_value(&mut self, node: Node<AstExpr>, rs: &mut RegScope) -> LValue {
    // 句柄借用：base 只读，整棵树经句柄门面判型，无槽位裸指针。
    let base = &node.get().base;

    self.set_debug_line_ast_node(base);

    // 句柄判型：命中即动态类型 AstExprLocal，且与 `node` 指向同一 arena 节点。
    if let Some(expr) = node.try_as::<AstExprLocal>() {
      let local_node = Node::from(expr.local);
      let local = local_node.get();
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
      if let Some(reg) = self.get_expr_local_reg(node) {
        LValue {
          kind: Kind::Local,
          reg,
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
    // 句柄判型：命中即动态类型 AstExprGlobal 的共享借用，name 字段只读。
    } else if let Some(expr) = node.try_as::<AstExprGlobal>() {
      if DebugLuauUserDefinedClasses.get()
        && let Some(&class_local) = self.class_locals.find(&expr.name)
      {
        CompileError::raise(
          &expr.base.base.location,
          core::format_args!(
            "'{}' refers to a class and cannot be used as a variable name (defined on line {})",
            sref_ast_name(expr.name),
            // class_locals 登记项为 parser 接线的 arena 存活节点句柄。
            class_local.get().location.begin.line + 1
          ),
        );
      }

      LValue {
        kind: Kind::Global,
        name: sref_ast_name(expr.name),
        location: base.location,
        ..Default::default()
      }
    // 句柄判型：命中即动态类型 AstExprIndexName；expr 子槽已句柄化恒非空，
    // `.get()` 直供 compile_expr_auto 只读借用。
    } else if let Some(expr) = node.try_as::<AstExprIndexName>() {
      LValue {
        kind: Kind::IndexName,
        reg: self.compile_expr_auto(expr.expr.get(), rs),
        name: sref_ast_name(expr.index),
        location: base.location,
        ..Default::default()
      }
    // 句柄判型：命中即动态类型 AstExprIndexExpr；expr.expr/index 已句柄化恒非空。
    } else if let Some(expr) = node.try_as::<AstExprIndexExpr>() {
      let reg = self.compile_expr_auto(expr.expr.get(), rs);
      let index = Node::from(expr.index);
      self.compile_l_value_index(reg, index, rs)
    } else {
      LUAU_ASSERT!(false);
      LValue {
        kind: Kind::Local,
        location: base.location,
        ..Default::default()
      }
    }
  }

  /// 对应 cpp `Compiler::compileLValueIndex`。原裸指针契约 fn 已 safe 化：
  /// `reg` 为当前窗口内已分配、持有被索引表的寄存器编号（SETTABLE/GETTABLE 的
  /// 基址槽）；`index` 为 parser 记录的 `AstExpr` 下标节点句柄（arena 存活、
  /// 非空证明内建）；`rs` 为宿主编译作用域的 RegScope，本函数
  /// 在其内分配 key/value 临时寄存器。
  pub(crate) fn compile_l_value_index(
    &mut self,
    reg: u8,
    index: Node<AstExpr>,
    rs: &mut RegScope,
  ) -> LValue {
    let index_node = index.get();
    let base = &index_node.base;
    let cv = self.get_constant(index_node);
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
        index: self.compile_expr_auto(index_node, rs),
        location: base.location,
        ..Default::default()
      },
    }
  }

  /// C++ `compileLValueUse`。`target_expr` 为赋值/取值目标节点句柄，
  /// 缺省（原 null 哨兵）以 `None` 表达；仅用于 hint 分支的 RTTI 判型，不解引用。
  pub(crate) fn compile_l_value_use(
    &mut self,
    lv: &LValue,
    reg: u8,
    set: bool,
    target_expr: Option<Node<AstExpr>>,
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
          && let Some(target_node) = target_expr.as_ref()
          && let Some(target_index_name) = target_node.try_as::<AstExprIndexName>()
        {
          self.hint_table_reg(target_index_name.expr, lv.reg, 2);
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

        // target_expr 只做 RTTI 判型（不解引用）：句柄命中即动态类型
        // AstExprIndexExpr，expr 子句柄存活，hint_* 只读。
        if let Some(target_node) = target_expr.as_ref()
          && let Some(target_index_expr) = target_node.try_as::<AstExprIndexExpr>()
        {
          // expr 已句柄化；hint_table_reg 收节点身份句柄，直接传句柄。
          self.hint_table_reg(target_index_expr.expr, lv.reg, 1);
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

        // target_expr 只做 RTTI 判型（不解引用）：句柄命中即动态类型
        // AstExprIndexExpr，expr/index 子句柄存活，hint_* 只读。
        if let Some(target_node) = target_expr.as_ref()
          && let Some(target_index_expr) = target_node.try_as::<AstExprIndexExpr>()
        {
          // expr/index 已句柄化；hint_* 收节点身份句柄，直接传句柄。
          self.hint_table_reg(target_index_expr.expr, lv.reg, 1);
          self.hint_number_reg(target_index_expr.index, lv.index);
        }
      }
    }
  }

  /// 编译类声明语句，对应 cpp `compileClassDeclaration`
  /// （cpp/Compiler/src/Compiler.cpp:1709）：`decl` 为分发器判型后调用方持有的
  /// arena 存活 `AstStatClass`（`name` 由 parser 保证非空）；须在
  /// `DebugLuauUserDefinedClasses` 旗标开启时调用（入口断言）。方法体子指针
  /// 交 `compile_expr_function`，其 arena 存活契约沿链传递。
  pub(crate) fn compile_class_declaration(&mut self, decl_ref: &AstStatClass) {
    LUAU_ASSERT!(fflag::DebugLuauUserDefinedClasses.get());

    // 类声明编译全程对 AST 只读，无其他写者；name 由 parser 保证非空接线。
    let name_ref = ast_slot_ref(decl_ref.name).expect("类名 AstLocal 为 parser 接线，恒非空");
    let class_name = name_ref.name;

    let class_local = self.class_locals.find(&class_name).copied();
    let dest = match class_local.and_then(|l| self.get_local_reg(l)) {
      Some(r) => r,
      None => {
        let d = self.alloc_reg(&decl_ref.base.base, 1);
        self.push_local(name_ref, d, u32::MAX);
        d
      }
    };

    if fflag::LuauExportValueSyntax.get() && decl_ref.exported {
      self.ensure_export_table(&decl_ref.base.base);
      // cpp `exportedClasses[decl->name] = dest`（Compiler.cpp:1732）：键为
      // 类局部 AstLocal 地址句柄——同名遮蔽的各 local 是独立键，不做按名去重
      self
        .exported_classes
        .push((Node::from(decl_ref.name), dest));
    }

    let _rs = self.reg_scope();

    let instr_c_val: u8 = u8::from(decl_ref.open);
    // 可空 super_ 槽 → Option 形态匹配（§2）：句柄在场即有父类表达式，缺席走
    // INVALID_SUPER_REG；解引用统一经句柄显式 borrow。
    match Node::try_new(decl_ref.super_) {
      Some(mut super_node) => {
        if let Some(super_reg) = self.get_expr_local_reg(super_node) {
          self
            .bc_mut()
            .emit_abc(LuauOpcode::LOP_NEWCLASS, dest, super_reg, instr_c_val);
        } else {
          let super_dest = self.alloc_reg(&decl_ref.base.base, 1);
          self.compile_expr(super_node.get_mut(), super_dest, false);
          self
            .bc_mut()
            .emit_abc(LuauOpcode::LOP_NEWCLASS, dest, super_dest, instr_c_val);
        }
      }
      None => {
        self.bc_mut().emit_abc(
          LuauOpcode::LOP_NEWCLASS,
          dest,
          INVALID_SUPER_REG,
          instr_c_val,
        );
      }
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
          self.check_constant(
            cid,
            &ast_slot_ref(method.function)
              .expect("method.function 为 parser 接线存活节点")
              .base
              .base
              .location,
          );
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
  /// `node` 为调用方持有的 arena 存活语句句柄（非空由 [`Node`] 承载），全程只读判型。
  pub(crate) fn is_stat_break(&self, node: Node<AstStat>) -> bool {
    // 句柄判型+下转：命中即 node 确为 AstStatBlock（repr(C) 首字段一致）。
    if let Some(block) = node.try_as::<AstStatBlock>() {
      // len == 1 守卫下界内取 body 首槽，同形只读判型
      block.body.len() == 1 && block.body.at(0).is::<AstStatBreak>()
    } else {
      node.is::<AstStatBreak>()
    }
  }

  /// C++ `extractStatContinue`：单语句块的 continue 语句节点，否则 `None`。
  /// 返回 [`Node<AstStatContinue>`] 仅作后续只读定位的地址句柄。
  pub(crate) fn extract_stat_continue(
    &self,
    block: &AstStatBlock,
  ) -> Option<Node<AstStatContinue>> {
    let body = &block.body;
    if body.len() != 1 {
      return None;
    }
    // len==1 守卫下界内取首槽（非空由 Node 类型端承载），
    // 句柄只读判型：block 借用证明内存活，class 校验后 Some 结果确为 AstStatContinue。
    body.at(0).try_as::<AstStatContinue>().map(Node::from_ref)
  }

  /// 校验 `continue` 跳过的 local 随后未被 `until` 条件使用；`start` 为
  /// continue 语句在循环体内的下标。
  pub(crate) fn validate_continue_until(
    &mut self,
    cont: &AstStat,
    mut condition: Node<AstExpr>,
    body: &AstStatBlock,
    start: usize,
  ) {
    let mut visitor = UndefinedLocalVisitor {
      compiler: self,
      undef: None,
      locals: DenseHashSet::default(),
    };

    for body_stat in body.body.iter_nodes().skip(start) {
      match body_stat.as_stat_ref() {
        AstStatRef::Local(local_stat) => {
          for &var in local_stat.vars.iter() {
            visitor.locals.insert(var.into());
          }
        }
        AstStatRef::LocalFunction(func_stat) => {
          visitor.locals.insert(func_stat.name.into());
        }
        _ => {}
      }
    }

    // 句柄升为独占借用后走安全引用门面；visitor 只写自身 locals/undef 集合、不写 AST。
    ast_expr_visit_ref(condition.get_mut(), &mut visitor);

    if let Some(undef) = visitor.undef {
      // undef 由 visitor 从子树收集的存活句柄（契约见 `Node::borrow`）。
      let undef_name = undef.get().name;
      CompileError::raise(
        &condition.get().base.location,
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
    // cpp Compiler.cpp:4190-4203：from/to/step 任一非数值常量、或 trip count
    // 无解，均落到同一条 remark——三处复读收成单点上报。step 与 cpp 同为
    // 无条件求值（get_constant 纯查表，无副作用差异）。
    let unroll = match (
      self.get_constant(stat_ref.from),
      self.get_constant(stat_ref.to),
      stat_ref.step.to_option().map(|s| self.get_constant(s)),
    ) {
      (Constant::Number(from), Constant::Number(to), step) => match step {
        Some(Constant::Number(v)) => Some((from, to, v)),
        Some(_) => None,
        None => Some((from, to, 1.0)),
      },
      _ => None,
    }
    .and_then(|(from, to, step)| {
      get_trip_count(from, to, step).map(|trip_count| (trip_count, from, step))
    });

    let Some((trip_count, from, step)) = unroll else {
      return self.reject_with_remark(format_args!("loop unroll failed: invalid iteration count"));
    };

    if LuauCompileLoopUnrollZero.get() && trip_count == 0 {
      self
        .bc_mut()
        .add_debug_remark(format_args!("loop unroll succeeded: empty loop"));
      return true;
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
    let cost_model = model_cost(
      Node::from(stat_ref.body).cast::<AstNode>().get_mut(),
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
    self.compile_unrolled_for(stat_ref, trip_count, from, step);
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

      self.fold_constants(
        Node::from(stat_ref.body).cast::<AstNode>().get_mut(),
        iv == 0,
      );

      let iter_jumps = self.loop_jumps.len();
      self.compile_stat(Node::from(stat_ref.body).cast::<AstStat>());

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
