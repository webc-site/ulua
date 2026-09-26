//! `Compiler` 表达式编译族：`compile_expr` 分发与各类表达式、条件/比较跳转、
//! 寄存器填充与操作码选择（对照 cpp `Compiler.cpp` 的 compileExpr* / getJump* 段）。
use core::{
  mem::swap,
  ptr::{from_mut, from_ref},
};

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{
    ast_array::AstArray,
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_call::AstExprCall,
    ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
    ast_expr_varargs::AstExprVarargs,
  },
};
use ulua_bytecode::{
  methods::bytecode_builder_get_string_hash::bytecode_builder_get_string_hash,
  records::bytecode_builder::BytecodeBuilder,
};
use ulua_common::{
  enums::{
    luau_builtin_function::LuauBuiltinFunction, luau_bytecode_type::LuauBytecodeType,
    luau_opcode::LuauOpcode,
  },
  fflag,
  fflag::DebugLuauUserDefinedClasses,
  macros::luau_assert::LUAU_ASSERT,
};

use crate::{
  functions::{
    ast_slot_ref::{ast_slot_is, ast_slot_ref, ast_slot_try_as},
    get_builtin_info::get_builtin_info,
    is_compare_op::is_compare_op,
    is_constant::is_constant_true,
    sref_compiler::{sref_ast_array_u8, sref_ast_name},
  },
  records::{
    compile_error::{CompileError, ERR_EXCEEDED_CONSTANT_LIMIT},
    compiler::{
      Compiler, K_GETIMPORT_FLAG, K_MAX_AD_INDEX, K_MAX_IMPORT_ID, K_MAX_K_CONST_INDEX,
      K_MAX_TARGET_COUNT,
    },
    constant::Constant,
    node::Node,
    reg_scope::RegScope,
  },
};

impl Compiler {
  /// 表达式分发器。`node` 由调用方的 `&mut` 交接证明其存活且独占（动态 RTTI
  /// 类型由本函数各分支门面守门）；`target` 须是外层 `alloc_reg`/RegScope 水位
  /// 内已分配的寄存器编号——发射的字节码直接写该槽。
  pub(crate) fn compile_expr(&mut self, node: &mut AstExpr, target: u8, target_temp: bool) {
    // constants/expr_local_reg 等以节点地址为键，由借用还原（与 fold 登记的指针同源）。
    let node_ptr: *mut AstExpr = from_mut(node);

    self.set_debug_line_ast_node(&node.base);

    if self.options.coverage_level >= 2 && self.needs_coverage(&node.base) {
      self.bc_mut().emit_abc(LuauOpcode::LOP_COVERAGE, 0, 0, 0);
    }

    if let Some(cv) = self.constants.find(&node_ptr.into())
      && !cv.is_unknown()
    {
      let cv = *cv;
      self.compile_expr_constant(node, &cv, target);
      return;
    }

    match node.as_expr_ref() {
      AstExprRef::Group(expr_group) => {
        // Box 字段靠自动解引用强转即可，无需显式 `*`（clippy explicit_auto_deref）
        self.compile_expr(
          unsafe { &mut *expr_group.expr.as_ptr() },
          target,
          target_temp,
        );
      }
      AstExprRef::ConstantNil(_) => {
        self
          .bc_mut()
          .emit_abc(LuauOpcode::LOP_LOADNIL, target, 0, 0);
      }
      AstExprRef::ConstantBool(expr_bool) => {
        self
          .bc_mut()
          .emit_abc(LuauOpcode::LOP_LOADB, target, expr_bool.value as u8, 0);
      }
      AstExprRef::ConstantNumber(expr_number) => {
        let cid = self.bc_mut().add_constant_number(expr_number.value);
        self.emit_constant_load(&expr_number.base, target, cid);
      }
      AstExprRef::ConstantInteger(expr_integer) => {
        let cid = self.bc_mut().add_constant_integer(expr_integer.value);
        self.emit_constant_load(&expr_integer.base, target, cid);
      }
      AstExprRef::ConstantString(expr_string) => {
        let cid = self
          .bc_mut()
          .add_constant_string(sref_ast_array_u8(expr_string.value));
        self.emit_constant_load(&expr_string.base, target, cid);
      }
      AstExprRef::Local(expr_local) => {
        // local 槽已句柄化恒非空：.get() 安全借用（旧 expect 判空分支随类型消失）。
        let local = expr_local.local.get();
        let is_exported_class = self
          .exported_classes
          .iter()
          // 身份比较为既有裸指针形态，经 as_ptr 桥接。
          .any(|(exported_local, _)| *exported_local == expr_local.local.as_ptr());
        if fflag::LuauExportValueSyntax.get() && local.is_exported && !is_exported_class {
          let name = sref_ast_name(local.name);
          let cid = self.bc_mut().add_constant_string(name.clone());
          self.check_constant(cid, &expr_local.base.base.location);
          let local_ptr = &raw mut self.export_table_local;
          let table_reg = self.get_local_reg(local_ptr);
          if table_reg >= 0 {
            self.bc_mut().emit_abc(
              LuauOpcode::LOP_GETTABLEKS,
              target,
              table_reg as u8,
              bytecode_builder_get_string_hash(name) as u8,
            );
            self.bc_mut().emit_aux(cid as u32);
          } else {
            let upval =
              self.get_upval(ast_slot_ref(local_ptr).expect("export_table_local 字段地址恒有效"));
            self
              .bc_mut()
              .emit_abc(LuauOpcode::LOP_GETUPVAL, target, upval, 0);
            self.bc_mut().emit_abc(
              LuauOpcode::LOP_GETTABLEKS,
              target,
              target,
              bytecode_builder_get_string_hash(name) as u8,
            );
            self.bc_mut().emit_aux(cid as u32);
          }
        } else {
          let reg = self.get_expr_local_reg(node_ptr);
          if reg >= 0 {
            if self.options.optimization_level == 0 || target != reg as u8 {
              self
                .bc_mut()
                .emit_abc(LuauOpcode::LOP_MOVE, target, reg as u8, 0);
            }
          } else {
            LUAU_ASSERT!(expr_local.upvalue);
            let uid = self.get_upval(local);
            self
              .bc_mut()
              .emit_abc(LuauOpcode::LOP_GETUPVAL, target, uid, 0);
          }
        }
      }
      AstExprRef::Global(expr_global) => {
        self.compile_expr_global(expr_global, target);
      }
      AstExprRef::Varargs(expr_varargs) => {
        self.compile_expr_varargs(expr_varargs, target, 1, false);
      }
      AstExprRef::Call(expr_call) => {
        if target_temp && self.reg_top != 0 && u32::from(target) == self.reg_top - 1 {
          self.compile_expr_call(expr_call, target, 1, true, false);
        } else {
          self.compile_expr_call(expr_call, target, 1, false, false);
        }
      }
      AstExprRef::IndexName(expr_index_name) => {
        self.compile_expr_index_name(expr_index_name, target, target_temp);
      }
      AstExprRef::IndexExpr(expr_index_expr) => {
        self.compile_expr_index_expr(expr_index_expr, target);
      }
      AstExprRef::Function(expr_function) => {
        self.compile_expr_function(expr_function, target);
      }
      AstExprRef::Table(expr_table) => {
        self.compile_expr_table(expr_table, target, target_temp);
      }
      AstExprRef::Unary(expr_unary) => {
        self.compile_expr_unary(expr_unary, target);
      }
      AstExprRef::Binary(expr_binary) => {
        self.compile_expr_binary(expr_binary, target, target_temp);
      }
      AstExprRef::TypeAssertion(expr_assertion) => {
        // expr 已句柄化恒非空；本入口持共享借用（cpp 非 const 透传形态），&mut 重建
        // 经 as_ptr 裸出口，写穿仅落在该节点编译期临时字段，单线程独占。
        self.compile_expr(
          unsafe { &mut *expr_assertion.expr.as_ptr() },
          target,
          target_temp,
        );
      }
      AstExprRef::IfElse(expr_if_else) => {
        self.compile_expr_if_else(expr_if_else, target, target_temp);
      }
      AstExprRef::InterpString(interp_string) => {
        self.compile_expr_interp_string(interp_string, target, target_temp);
      }
      AstExprRef::Instantiate(expr_instantiate) => {
        self.compile_expr(unsafe { &mut *expr_instantiate.expr }, target, target_temp);
      }
      AstExprRef::Error(_) => LUAU_ASSERT!(false),
    }
  }

  /// 对应 cpp `compileExprAndOr`（cpp/Compiler/src/Compiler.cpp:2259）：短路
  /// 与/或的指令化（AND/ANDK/OR/ORK 或条件跳转链）。`expr` 为分发器判型后调用方
  /// 持有的存活 `AstExprBinary`（op ∈ {And, Or}），其 `left`/`right` 子指针全程
  /// 只读消费；`target` 须为已分配寄存器编号（短路分支的 jump patch 与赋值均写该槽）。
  pub(crate) fn compile_expr_and_or(
    &mut self,
    expr: &AstExprBinary,
    target: u8,
    target_temp: bool,
  ) {
    {
      let expr_ref = expr;
      let and_ = expr_ref.op == AstExprBinaryOp::And;
      let mut rs = self.reg_scope();

      if let Some(cl) = self.constants.find(&expr_ref.left.into())
        && !cl.is_unknown()
      {
        // left/right 已句柄化；本入口持 &AstExprBinary（cpp 非 const 透传形态），
        // &mut 重建仅落在被选中一侧节点的编译期临时字段，另一侧只以句柄地址作 constants 键。
        // Safety: branch 出自 parser 接线的存活 arena 子句柄（NonNull 恒非空），
        // 编译期对该子树单线程独占写穿，与旧 `&mut *(...)` 形态同一契约。
        let branch = if and_ == cl.is_truthful() {
          expr_ref.right
        } else {
          expr_ref.left
        };
        self.compile_expr(unsafe { &mut *branch.as_ptr() }, target, target_temp);
        return;
      }

      // left/right 已句柄化；is_condition_fast 为既有裸指针 API 经 as_ptr 桥接，
      // get_expr_local_reg 收 impl Into<Node> 直接喂句柄。
      if !self.is_condition_fast(expr_ref.left.as_ptr()) {
        if let reg = self.get_expr_local_reg(expr_ref.right)
          && reg >= 0
        {
          let lr = self.compile_expr_auto(expr_ref.left.as_ptr(), &mut rs);
          self.bc_mut().emit_abc(
            if and_ {
              LuauOpcode::LOP_AND
            } else {
              LuauOpcode::LOP_OR
            },
            target,
            lr,
            reg as u8,
          );
          return;
        }

        // 门面解引用：right 已句柄化恒非空，as_ptr 喂既有指针门面，get_constant_index 只读消费。
        let cid = self.get_constant_index(
          ast_slot_ref(expr_ref.right.as_ptr()).expect("right 为 parser 接线的存活子指针"),
        );
        if (0..K_MAX_K_CONST_INDEX).contains(&cid) {
          let lr = self.compile_expr_auto(expr_ref.left.as_ptr(), &mut rs);
          self.bc_mut().emit_abc(
            if and_ {
              LuauOpcode::LOP_ANDK
            } else {
              LuauOpcode::LOP_ORK
            },
            target,
            lr,
            cid as u8,
          );
          return;
        }
      }

      let reg = if target_temp {
        target
      } else {
        self.alloc_reg(&expr_ref.base.base, 1)
      };
      // left 已句柄化；compile_condition_value 为既有裸指针 API 经 as_ptr 桥接。
      // Safety: right 出自 parser 接线的存活 arena 子句柄；本入口持 &AstExprBinary
      // （cpp 非 const 透传形态），&mut 重建限于该节点编译期临时字段，单线程独占。
      let skip_jump = self.compile_condition_value(expr_ref.left.as_ptr(), Some(reg), !and_);
      self.compile_expr(unsafe { &mut *expr_ref.right.as_ptr() }, reg, true);
      let move_label = self.bc().emit_label();
      self.patch_jumps(&expr_ref.base.base, &skip_jump, move_label);

      if target != reg {
        self.bc_mut().emit_abc(LuauOpcode::LOP_MOVE, target, reg, 0);
      }
    }
  }

  /// 对应 cpp `compileExprAuto`（cpp/Compiler/src/Compiler.cpp:3366）：求值表达式
  /// 到自动分配的寄存器并返回其编号。`node` 为 parser 接线的 arena 存活节点
  /// （`compile_or_throw` 持有 parse arena 至编译结束），本函数只读取基类头与转发
  /// 地址键。`_rs` 仅为寄存器作用域见证：调用方须持有覆盖返回值寄存器分配的
  /// RegScope，返回值是 `alloc_reg` 分配的新寄存器编号，其有效性依赖该守卫尚未回卷。
  pub(crate) fn compile_expr_auto(&mut self, node: *mut AstExpr, _rs: &mut RegScope) -> u8 {
    // 门面无借用解引用：node 为 parser 接线的 arena 存活节点（见上）。
    let node_ref = ast_slot_ref(node).expect("node 为 parser 接线的 arena 存活节点");
    let reg = self.get_expr_local_reg(node);
    if reg >= 0 {
      return reg as u8;
    }

    let reg = self.alloc_reg(&node_ref.base, 1);
    // Safety: 上面已解引用确认 node 为可读的 arena 存活节点；`node_ref` 的共享
    // 借用止于 alloc_reg，此后重建 `&mut` 交分发器，无别名重叠。
    self.compile_expr(unsafe { &mut *node }, reg, true);
    reg
  }

  /// 对应 cpp `compileExprBinary`（cpp/Compiler/src/Compiler.cpp:2325 附近二元
  /// 分发）：算术/拼接/比较的指令化。`expr` 为分发器判型后调用方持有的存活
  /// `AstExprBinary`；`left`/`right` 与 concat args 子指针仅传给只读消费路径；
  /// 节点地址仅作 expr_types 哈希键、不解引用。`target` 须为已分配寄存器编号（结果槽）。
  pub(crate) fn compile_expr_binary(
    &mut self,
    expr: &AstExprBinary,
    target: u8,
    _target_temp: bool,
  ) {
    {
      let expr_ref = expr;
      let mut rs = self.reg_scope();

      match expr_ref.op {
        AstExprBinaryOp::Add
        | AstExprBinaryOp::Sub
        | AstExprBinaryOp::Mul
        | AstExprBinaryOp::Div
        | AstExprBinaryOp::FloorDiv
        | AstExprBinaryOp::Mod
        | AstExprBinaryOp::Pow => {
          // 门面解引用：right 已句柄化恒非空，as_ptr 喂既有指针门面，get_constant_number 只读。
          let rc = self.get_constant_number(
            ast_slot_ref(expr_ref.right.as_ptr()).expect("right 为 parser 接线的存活子指针"),
          );
          if (0..K_MAX_K_CONST_INDEX).contains(&rc) {
            let rl = self.compile_expr_auto(expr_ref.left.as_ptr(), &mut rs);
            let op = self.get_binary_op_arith(expr_ref.op, true);
            self.bc_mut().emit_abc(op, target, rl, rc as u8);
            self.hint_number_reg(expr_ref.left.as_ptr(), rl);
          } else {
            if expr_ref.op == AstExprBinaryOp::Sub || expr_ref.op == AstExprBinaryOp::Div {
              // 门面解引用：left 已句柄化恒非空，as_ptr 喂既有指针门面，只读常量折叠。
              let lc = self.get_constant_number(
                ast_slot_ref(expr_ref.left.as_ptr()).expect("left 为 parser 接线的存活子指针"),
              );
              if (0..K_MAX_K_CONST_INDEX).contains(&lc) {
                let rr = self.compile_expr_auto(expr_ref.right.as_ptr(), &mut rs);
                let op = if expr_ref.op == AstExprBinaryOp::Sub {
                  LuauOpcode::LOP_SUBRK
                } else {
                  LuauOpcode::LOP_DIVRK
                };
                self.bc_mut().emit_abc(op, target, lc as u8, rr);
                self.hint_number_reg(expr_ref.right.as_ptr(), rr);
                return;
              }
            } else if self.options.optimization_level >= 2
              && (expr_ref.op == AstExprBinaryOp::Add || expr_ref.op == AstExprBinaryOp::Mul)
            {
              // 优化：r 已知为数字（否则可能触发元方法）时把 k*r 换成 r*k。
              // 向量只对乘法成立，因为 number+vector 是错误。
              if let Some(ty) = self
                .expr_types
                .find(&Node::from_ref(expr).cast::<AstExpr>())
                .copied()
                && (ty == LuauBytecodeType::LBC_TYPE_NUMBER
                  || (ty == LuauBytecodeType::LBC_TYPE_VECTOR
                    && expr_ref.op == AstExprBinaryOp::Mul))
              {
                // 门面解引用：left 已句柄化恒非空，as_ptr 喂既有指针门面，只读常量折叠。
                let lc = self.get_constant_number(
                  ast_slot_ref(expr_ref.left.as_ptr()).expect("left 为 parser 接线的存活子指针"),
                );
                if (0..K_MAX_K_CONST_INDEX).contains(&lc) {
                  let rr = self.compile_expr_auto(expr_ref.right.as_ptr(), &mut rs);
                  let op = self.get_binary_op_arith(expr_ref.op, true);
                  self.bc_mut().emit_abc(op, target, rr, lc as u8);
                  self.hint_number_reg(expr_ref.right.as_ptr(), rr);
                  return;
                }
              }
            }
            let rl = self.compile_expr_auto(expr_ref.left.as_ptr(), &mut rs);
            let rr = self.compile_expr_auto(expr_ref.right.as_ptr(), &mut rs);
            let op = self.get_binary_op_arith(expr_ref.op, false);
            self.bc_mut().emit_abc(op, target, rl, rr);
            self.hint_number_reg(expr_ref.left.as_ptr(), rl);
            self.hint_number_reg(expr_ref.right.as_ptr(), rr);
          }
        }
        AstExprBinaryOp::Concat => {
          // args 行走链为既有裸指针 API，句柄入口经 as_ptr 桥接。
          let mut args = vec![expr_ref.left.as_ptr(), expr_ref.right.as_ptr()];
          self.unroll_concats(&mut args);
          let regs = self.alloc_reg(&expr_ref.base.base, args.len() as u32);
          for (i, &arg) in args.iter().enumerate() {
            // Safety: args 各项为 parser 接线的存活 AstExpr 子指针（unroll_concats 仅
            // 收集同树节点）；&mut 写穿限于该节点编译期临时字段。
            if fflag::LuauCompileConcatTargetTop.get() {
              self.compile_expr_temp_top(unsafe { &mut *arg }, regs + i as u8);
            } else {
              self.compile_expr(unsafe { &mut *arg }, regs + i as u8, true);
            }
          }
          self.bc_mut().emit_abc(
            LuauOpcode::LOP_CONCAT,
            target,
            regs,
            regs + args.len() as u8 - 1,
          );
        }
        op if is_compare_op(op) => {
          let jump_label = self.compile_compare_jump(expr_ref, false);
          self.bc_mut().emit_abc(LuauOpcode::LOP_LOADB, target, 0, 1);
          let then_label = self.bc().emit_label();
          self.bc_mut().emit_abc(LuauOpcode::LOP_LOADB, target, 1, 0);
          self.patch_jump(&expr_ref.base.base, jump_label, then_label);
        }
        AstExprBinaryOp::And | AstExprBinaryOp::Or => {
          self.compile_expr_and_or(expr_ref, target, _target_temp);
        }
        _ => LUAU_ASSERT!(false),
      }
    }
  }

  /// 对应 cpp `compileExprConstant`（cpp/Compiler/src/Compiler.cpp:3089）：把已折叠
  /// 常量直载到寄存器。`node` 仅在常量表溢出报错路径读取 `location`；`cv` 必须是
  /// `constants` 表中对该 node 已求得的折叠常量；`target` 须为已分配寄存器编号——
  /// LOADNIL/LOADB/加载指令直接写该槽。
  pub(crate) fn compile_expr_constant(&mut self, node: &AstExpr, cv: &Constant, target: u8) {
    match *cv {
      Constant::Nil => self
        .bc_mut()
        .emit_abc(LuauOpcode::LOP_LOADNIL, target, 0, 0),
      Constant::Boolean(b) => self
        .bc_mut()
        .emit_abc(LuauOpcode::LOP_LOADB, target, b as u8, 0),
      Constant::Number(d) => {
        let fits_i16 = d >= (i16::MIN as f64)
          && d <= (i16::MAX as f64)
          && (d as i16 as f64) == d
          && !(d == 0.0 && d.is_sign_negative());

        if fits_i16 {
          self
            .bc_mut()
            .emit_ad(LuauOpcode::LOP_LOADN, target, d as i16);
        } else {
          let cid = self.bc_mut().add_constant_number(d);
          self.emit_constant_load(node, target, cid);
        }
      }
      Constant::Integer(l) => {
        let cid = self.bc_mut().add_constant_integer(l);
        self.emit_constant_load(node, target, cid);
      }
      Constant::Vector([x, y, z, w]) => {
        let cid = self.bc_mut().add_constant_vector(x, y, z, w);
        self.emit_constant_load(node, target, cid);
      }
      Constant::Str(_) => {
        let s = cv.get_string();
        let cid = self.bc_mut().add_constant_string(sref_ast_array_u8(s));
        self.emit_constant_load(node, target, cid);
      }
      // 仅 Unknown / Table 会到达
      _ => {
        LUAU_ASSERT!(false);
      }
    }
  }

  /// 常量索引溢出报错 + LOADK/LOADKX 发射的单点门面（cpp `emitLoadK2` 同形）：
  /// `node` 仅在溢出报错路径读取 `location`，`cid` 为 `addConstantXxx` 的返回值。
  #[inline]
  pub(crate) fn emit_constant_load(&mut self, node: &AstExpr, target: u8, cid: i32) {
    self.check_constant(cid, &node.base.location);
    self.emit_load_k(target, cid);
  }

  /// `target` 须为已分配寄存器编号（GETIMPORT/GETGLOBAL/用户类 local 搬运写该槽）。
  /// `class_locals` 命中的 `*mut AstLocal` 系构造期接线、比 self 长寿的存活节点。
  pub(crate) fn compile_expr_global(&mut self, expr: &AstExprGlobal, target: u8) {
    if DebugLuauUserDefinedClasses.get()
      && let Some(&local) = self.class_locals.find(&expr.name)
    {
      let reg = self.get_local_reg(local);
      if reg >= 0 {
        if target != reg as u8 {
          self
            .bc_mut()
            .emit_abc(LuauOpcode::LOP_MOVE, target, reg as u8, 0);
        }
      } else {
        // local 是 class_locals 表登记的存活节点句柄（构造期登记、比 self 长寿），
        // get_upval 常态仅按地址查重。
        let uid = self.get_upval(local.borrow());
        self
          .bc_mut()
          .emit_abc(LuauOpcode::LOP_GETUPVAL, target, uid, 0);
      }
      return;
    }

    if self.can_import(expr) {
      let name = expr.name;
      let id0 = self.bc_mut().add_constant_string(sref_ast_name(name));
      self.check_constant(id0, &expr.base.base.location);

      if id0 < K_MAX_IMPORT_ID {
        let iid = BytecodeBuilder::get_import_id(id0);
        let cid = self.bc_mut().add_import(iid);

        if (0..K_MAX_AD_INDEX).contains(&cid) {
          self.emit_ad_aux(LuauOpcode::LOP_GETIMPORT, target, cid as i16, iid);
          return;
        }
      }
    }

    let name = expr.name;
    let gname = sref_ast_name(name);
    let cid = self.bc_mut().add_constant_string(gname.clone());
    self.check_constant(cid, &expr.base.base.location);

    let hash = BytecodeBuilder::get_string_hash(gname) as u8;
    self.emit_abc_aux(LuauOpcode::LOP_GETGLOBAL, target, 0, hash, cid as u32);
  }

  /// `target` 须为已分配寄存器编号，`target_temp=true` 表示调用方允许覆写该槽；
  /// condition/true_expr/false_expr 已句柄化恒非空（缺 else 注入 nil 常量节点）。
  pub(crate) fn compile_expr_if_else(
    &mut self,
    expr_ref: &AstExprIfElse,
    target: u8,
    target_temp: bool,
  ) {
    if self.is_constant(expr_ref.condition.as_ptr()) {
      if is_constant_true(&self.constants, expr_ref.condition.as_ptr().into()) {
        // Safety: true_expr 已句柄化（parser 接线的存活子节点）；本函数持共享引用，
        // 写穿借用的还原沿既有形态经 as_ptr 桥接，target 由上层分配到本帧寄存器。
        self.compile_expr(
          unsafe { &mut *expr_ref.true_expr.as_ptr() },
          target,
          target_temp,
        );
      } else {
        // Safety: false_expr 同上（句柄即存活证明）。
        self.compile_expr(
          unsafe { &mut *expr_ref.false_expr.as_ptr() },
          target,
          target_temp,
        );
      }
    } else {
      let creg = self.get_expr_local_reg(expr_ref.condition.as_ptr());
      if creg >= 0 {
        let true_reg = self.get_expr_local_reg(expr_ref.true_expr.as_ptr());
        let false_reg = self.get_expr_local_reg(expr_ref.false_expr.as_ptr());

        if creg == true_reg && (false_reg >= 0 || self.is_constant(expr_ref.false_expr.as_ptr())) {
          // creg 来自 get_expr_local_reg 的寄存器值；other=false_expr 已句柄化，
          // .get() 直接物化只读借用（判空 expect 随非空类型消失）。
          return self.compile_expr_if_else_and_or(
            false,
            creg as u8,
            expr_ref.false_expr.get(),
            target,
          );
        } else if creg == false_reg
          && (true_reg >= 0 || self.is_constant(expr_ref.true_expr.as_ptr()))
        {
          // 与上一分支对称，other=true_expr 句柄化后经 .get() 物化只读借用。
          return self.compile_expr_if_else_and_or(
            true,
            creg as u8,
            expr_ref.true_expr.get(),
            target,
          );
        }
      }

      // 返回值即本条件新发射的 skip 跳转标签；condition 句柄经 as_ptr 喂既有裸指针 API。
      let else_jump = self.compile_condition_value(expr_ref.condition.as_ptr(), None, false);
      // Safety: true_expr 存活（同上）。
      self.compile_expr(
        unsafe { &mut *expr_ref.true_expr.as_ptr() },
        target,
        target_temp,
      );

      // 不驻留 &mut 长借用（原写法的局部别名会与后续 &mut self 调用重叠），
      // 逐点经 bc/bc_mut 取现，语义同 cpp 的引用成员直调。
      let then_label = self.bc().emit_label();
      self.bc_mut().emit_ad(LuauOpcode::LOP_JUMP, 0, 0);

      let else_label = self.bc().emit_label();
      // Safety: false_expr 存活（同上）。
      self.compile_expr(
        unsafe { &mut *expr_ref.false_expr.as_ptr() },
        target,
        target_temp,
      );
      let end_label = self.bc().emit_label();

      self.patch_jumps(&expr_ref.base.base, &else_jump, else_label);
      self.patch_jump(&expr_ref.base.base, then_label, end_label);
    }
  }

  /// `creg`/`target` 须为当前寄存器窗口内已分配编号（分支落点与合并写槽）；
  /// `other` 为 `compile_expr_if_else` 传入的另一侧操作数。
  pub(crate) fn compile_expr_if_else_and_or(
    &mut self,
    and_: bool,
    creg: u8,
    other: &AstExpr,
    target: u8,
  ) {
    // 借用期内 other 存活有类型证明；还原为地址喂给以指针为键的 map 与只读消费链。
    let other_ptr: *mut AstExpr = from_ref(other).cast_mut();

    let cid = self.get_constant_index(other);
    if (0..K_MAX_K_CONST_INDEX).contains(&cid) {
      self.bc_mut().emit_abc(
        if and_ {
          LuauOpcode::LOP_ANDK
        } else {
          LuauOpcode::LOP_ORK
        },
        target,
        creg,
        cid as u8,
      );
    } else {
      let mut rs = self.reg_scope();
      let oreg = self.compile_expr_auto(other_ptr, &mut rs);
      self.bc_mut().emit_abc(
        if and_ {
          LuauOpcode::LOP_AND
        } else {
          LuauOpcode::LOP_OR
        },
        target,
        creg,
        oreg,
      );
    }
  }

  /// `target` 须为已分配寄存器编号（GETTABLE 结果槽）；子指针 `expr`/`index` 为
  /// parser 接线、编译期存活的 `AstExpr` 节点，全部经只读消费路径下传。
  pub(crate) fn compile_expr_index_expr(&mut self, expr: &AstExprIndexExpr, target: u8) {
    let mut rs = self.reg_scope();
    // expr/index 已句柄化恒非空；get_constant/hint_*/compile_expr_auto 为既有裸指针 API，
    // 经 as_ptr 桥接（只读消费路径不变）。
    let cv = self.get_constant(expr.index.as_ptr());
    // 门面解引用：下方各分支只在 index 命中常量（即 parser 登记于 constants 表的
    // 存活字面量节点）时读取其 location，指针编译期稳定。
    let index_location = &ast_slot_ref(expr.index.as_ptr())
      .expect("expr.index 为 parser 接线的存活子指针")
      .base
      .location;

    match cv {
      // 整数下标 1..=256 走 GETTABLEN 快路径
      Constant::Number(n) if (1.0..=256.0).contains(&n) && (n as i32 as f64) == n => {
        let i = (n as i32 - 1) as u8;
        let rt = self.compile_expr_auto(expr.expr.as_ptr(), &mut rs);
        self.set_debug_line_location(index_location);
        self
          .bc_mut()
          .emit_abc(LuauOpcode::LOP_GETTABLEN, target, rt, i);
        self.hint_table_reg(expr.expr.as_ptr(), rt, 1);
      }
      Constant::Str(_) => {
        let iname = sref_ast_array_u8(cv.get_string());
        let cid = self.bc_mut().add_constant_string(iname.clone());
        self.check_constant(cid, &expr.base.base.location);
        let rt = self.compile_expr_auto(expr.expr.as_ptr(), &mut rs);
        self.set_debug_line_location(index_location);
        self.bc_mut().emit_abc(
          LuauOpcode::LOP_GETTABLEKS,
          target,
          rt,
          bytecode_builder_get_string_hash(iname) as u8,
        );
        self.bc_mut().emit_aux(cid as u32);
        self.hint_table_reg(expr.expr.as_ptr(), rt, 2);
      }
      _ => {
        let rt = self.compile_expr_auto(expr.expr.as_ptr(), &mut rs);
        let ri = self.compile_expr_auto(expr.index.as_ptr(), &mut rs);
        self
          .bc_mut()
          .emit_abc(LuauOpcode::LOP_GETTABLE, target, rt, ri);
        self.hint_table_reg(expr.expr.as_ptr(), rt, 1);
        self.hint_number_reg(expr.index.as_ptr(), ri);
      }
    }
  }

  /// `target` 须为已分配寄存器编号（GETTABLEKS/GETIMPORT 结果槽），
  /// `target_temp=true` 表示调用方允许覆写该槽；子指针 `expr` 为 parser 接线、
  /// 编译期存活的 `AstExpr` 节点。
  pub(crate) fn compile_expr_index_name(
    &mut self,
    expr: &AstExprIndexName,
    target: u8,
    target_temp: bool,
  ) {
    self.set_debug_line_ast_node(&expr.base.base);

    let (import_root, import1, import2): (
      Option<&AstExprGlobal>,
      &AstExprIndexName,
      Option<&AstExprIndexName>,
    ) = {
      // 门面判型+下转：expr.expr 已句柄化恒非空，经 as_ptr 桥接进槽位门面，
      // 判空/类型不符返回 None，命中即类型正确。
      if let Some(index) = ast_slot_try_as::<AstExprIndexName, _>(expr.expr.as_ptr()) {
        // 同款门面契约，index.expr 已句柄化恒非空，经 as_ptr 桥接。
        let root = ast_slot_try_as::<AstExprGlobal, _>(index.expr.as_ptr());
        (root, index, Some(expr))
      } else {
        // 同款门面契约，expr.expr 已句柄化恒非空，经 as_ptr 桥接。
        let root = ast_slot_try_as::<AstExprGlobal, _>(expr.expr.as_ptr());
        (root, expr, None)
      }
    };

    if let Some(root) = import_root
      && self.can_import_chain(root)
      && !(DebugLuauUserDefinedClasses.get() && self.class_locals.find(&root.name).is_some())
    {
      let id0 = self.bc_mut().add_constant_string(sref_ast_name(root.name));
      let id1 = self
        .bc_mut()
        .add_constant_string(sref_ast_name(import1.index));
      let id2 = import2.map_or(-1, |i2| {
        self.bc_mut().add_constant_string(sref_ast_name(i2.index))
      });

      if id0 < 0 || id1 < 0 || (import2.is_some() && id2 < 0) {
        CompileError::raise(
          &expr.base.base.location,
          format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
        );
      }

      // 注意：GETIMPORT 编码里每个对象 id 分量限 10 bit
      if id0 < K_MAX_IMPORT_ID && id1 < K_MAX_IMPORT_ID && id2 < K_MAX_IMPORT_ID {
        let iid = match import2 {
          Some(_) => BytecodeBuilder::get_import_id3(id0, id1, id2),
          None => BytecodeBuilder::get_import_id2(id0, id1),
        };
        let cid = self.bc_mut().add_import(iid);
        if (0..K_MAX_AD_INDEX).contains(&cid) {
          self.emit_ad_aux(LuauOpcode::LOP_GETIMPORT, target, cid as i16, iid);
          return;
        }
      }
    }

    let mut rs = self.reg_scope();
    let local_reg = self.get_expr_local_reg(expr.expr);
    let reg = if local_reg >= 0 {
      local_reg as u8
    } else if target_temp {
      // Safety: expr.expr 已句柄化恒非空（上方同款契约），裸出口经 as_ptr 桥接写穿
      // 子槽位；target 本帧可覆写。
      self.compile_expr(unsafe { &mut *expr.expr.as_ptr() }, target, true);
      target
    } else {
      self.compile_expr_auto(expr.expr.as_ptr(), &mut rs)
    };

    self.set_debug_line_location(&expr.index_location);
    let iname = sref_ast_name(expr.index);
    let cid = self.bc_mut().add_constant_string(iname.clone());
    self.check_constant(cid, &expr.base.base.location);

    self.bc_mut().emit_abc(
      LuauOpcode::LOP_GETTABLEKS,
      target,
      reg,
      bytecode_builder_get_string_hash(iname) as u8,
    );
    self.bc_mut().emit_aux(cid as u32);
    // expr 已句柄化恒非空；hint_table_reg 为既有裸指针 API，经 as_ptr 桥接。
    self.hint_table_reg(expr.expr.as_ptr(), reg, 2);
  }

  pub(crate) fn compile_expr_list_temp(
    &mut self,
    list: &AstArray<*mut AstExpr>,
    target: u8,
    target_count: u8,
    target_top: bool,
  ) {
    LUAU_ASSERT!(!target_top || (target as u32 + target_count as u32) == self.reg_top);

    // 以下各循环元素均由 parser 保证为非空存活 AstExpr 指针（list 来自语句/表达式的
    // AstArray 实参表），target.wrapping_add(i) 落在本帧已分配的连续窗口内。
    if list.size == target_count as usize {
      for (i, &expr) in list.iter().enumerate() {
        // Safety: 见上，expr 存活、寄存器有效。
        self.compile_expr(unsafe { &mut *expr }, target.wrapping_add(i as u8), true);
      }
    } else if list.size > target_count as usize {
      for (i, &expr) in list.iter().take(target_count as usize).enumerate() {
        // Safety: 见上，expr 存活、寄存器有效。
        self.compile_expr(unsafe { &mut *expr }, target.wrapping_add(i as u8), true);
      }

      for &expr in list.iter().skip(target_count as usize) {
        self.compile_expr_side(expr);
      }
    } else if !list.is_empty() {
      for (i, &expr) in list.iter().take(list.size - 1).enumerate() {
        // Safety: 见上，expr 存活、寄存器有效。
        self.compile_expr(unsafe { &mut *expr }, target.wrapping_add(i as u8), true);
      }

      // !is_empty 已保证末槽存在：as_slice().last() 取代裸 data.add(size-1)，
      // 无需 unsafe；None 为不可达防御分支（本函数尾段，早退无副作用）
      let Some(last_expr) = list.as_slice().last().copied() else {
        return;
      };
      self.compile_expr_temp_n(
        // last_expr 同上存活（parser 接线），&mut 借用只覆盖该次调用。
        unsafe { &mut *last_expr },
        target.wrapping_add((list.size - 1) as u8),
        target_count.wrapping_sub((list.size - 1) as u8),
        target_top,
      );
    } else {
      self.fill_regs_nil(target, 0, target_count);
    }
  }

  /// `node` 须为 arena 存活 `AstExpr` 基指针：判型门面与被调链（副作用表达式求值）
  /// 全程读取该地址；纯局部/常量表达式被跳过。无寄存器入参，寄存器分配由内部
  /// RegScope 自管。悬垂 node 使判型门面读 RTTI 头即 UB。
  pub(crate) fn compile_expr_side(&mut self, node: *mut AstExpr) {
    {
      // 门面判型（只读 class index，null 折叠 false）：node 契约即上方存活前提。
      if ast_slot_is::<AstExprLocal, _>(node)
        || ast_slot_is::<AstExprGlobal, _>(node)
        || ast_slot_is::<AstExprVarargs, _>(node)
        || ast_slot_is::<AstExprFunction, _>(node)
        || self.is_constant(node)
      {
        return;
      }

      // 门面判型：node 存活基指针同上。
      if !ast_slot_is::<AstExprCall, _>(node) {
        self
          .bc_mut()
          .add_debug_remark(format_args!("expression only compiled for side effects"));
      }

      let mut rsi = self.reg_scope();
      self.compile_expr_auto(node, &mut rsi);
    }
  }

  /// `target` 须为已分配寄存器编号；子表达式 `expr.expr` 为 parser 接线的存活
  /// `AstExpr` 指针，取负折叠分支的 RTTI 命中即动态类型为 `AstExprConstantInteger`。
  pub(crate) fn compile_expr_unary(&mut self, expr: &AstExprUnary, target: u8) {
    let mut rs = self.reg_scope();

    if fflag::LuauIntegerType2.get()
      && expr.op == AstExprUnaryOp::Minus
      // 门面判型+下转：expr.expr 已句柄化（parser 接线、编译期存活），as_ptr 喂既有
      // 判型门面，命中才读 cint.value。
      && let Some(cint) = ast_slot_try_as::<AstExprConstantInteger, _>(expr.expr.as_ptr())
    {
      // 二进制补码取负 `~v + 1 == -v`。此处 `+ 1` 必须回绕（C 语义）：
      // v == 0 时 `u64::MAX -> 0` 回绕、v == i64::MIN 时回绕到
      // i64::MIN——两者都正确。若用 checked `+`，在 fuzz 构建的
      // overflow-checks 下会 panic（编译期 fuzzer 在 `-<int 0>` 上发现）。
      let cid = self
        .bc_mut()
        .add_constant_integer((!(cint.value as u64)).wrapping_add(1) as i64);
      self.check_constant(cid, &expr.base.base.location);
      self.emit_load_k(target, cid);
      return;
    }

    // expr 已句柄化；compile_expr_auto 为既有裸指针 API，经 as_ptr 桥接。
    let re = self.compile_expr_auto(expr.expr.as_ptr(), &mut rs);
    let op = self.get_unary_op(expr.op);
    self.bc_mut().emit_abc(op, target, re, 0);
  }

  /// `target..target+target_count` 须为 MULTIEXPR 展开预留的连续寄存器区间，
  /// `target_count < K_MAX_TARGET_COUNT` 且 `mult_ret` 时须满足
  /// `target+target_count == reg_top`（入口断言校验）。
  pub(crate) fn compile_expr_varargs(
    &mut self,
    expr: &AstExprVarargs,
    target: u8,
    target_count: u8,
    mult_ret: bool,
  ) {
    LUAU_ASSERT!(target_count < K_MAX_TARGET_COUNT);
    LUAU_ASSERT!(!mult_ret || u32::from(target) + u32::from(target_count) == self.reg_top);

    self.set_debug_line_ast_node(&expr.base.base);

    self.bc_mut().emit_abc(
      LuauOpcode::LOP_GETVARARGS,
      target,
      if mult_ret {
        0
      } else {
        target_count.wrapping_add(1)
      },
      0,
    );
  }

  /// `target..target+target_count` 须落在已分配寄存器窗口内，且 `target_top=true`
  /// 时 `target+target_count == reg_top`（入口 LUAU_ASSERT 校验）；
  /// `target_count == K_MAX_TARGET_COUNT` 为哨兵值，走报错路径仅读 location。
  pub(crate) fn compile_expr_temp_n(
    &mut self,
    node: &mut AstExpr,
    target: u8,
    target_count: u8,
    target_top: bool,
  ) {
    LUAU_ASSERT!(!target_top || u32::from(target) + u32::from(target_count) == self.reg_top);

    if target_count == K_MAX_TARGET_COUNT {
      let location = node.base.location;
      CompileError::raise(
        &location,
        core::format_args!("Exceeded result count limit; simplify the code to compile"),
      );
    }

    // safe 下转门面：命中即动态类型为 AstExprCall 或 AstExprVarargs。
    match node.as_expr_ref() {
      AstExprRef::Call(expr_call) => {
        self.compile_expr_call(expr_call, target, target_count, target_top, false);
        return;
      }
      AstExprRef::Varargs(expr_varargs) => {
        self.compile_expr_varargs(expr_varargs, target, target_count, false);
        return;
      }
      _ => {}
    }

    self.compile_expr(node, target, true);

    self.fill_regs_nil(target, 1, target_count);
  }

  /// 以 `target+1` 为临时寄存器水位顶格编译表达式：守卫 `reg_scope_top`
  /// 在返回时回卷 `reg_top`，调用方须接受该窗口收缩；节点存活/独占与 `target`
  /// 已分配契约同 `compile_expr`。
  pub(crate) fn compile_expr_temp_top(&mut self, node: &mut AstExpr, target: u8) {
    let _rs = self.reg_scope_top(target as u32 + 1);
    self.compile_expr(node, target, true);
  }

  /// `target` 须为已分配寄存器编号；返回值指示是否按多返回值展开
  ///（reg 窗口被推移到 `target..reg_top`）。
  pub(crate) fn compile_expr_temp_mult_ret(&mut self, node: &mut AstExpr, target: u8) -> bool {
    // 裸地址快照：仅作 is_expr_mult_ret 的只读遍历入参。
    let node_ptr = from_mut(&mut *node);

    // Call 且非 multret 时按单值 temp 路径编译；判定先于 &mut 下转，
    // 避免 Call 借用横跨对同一 node 的递归分发。
    if self.options.optimization_level >= 2
      && matches!(node.as_expr_ref(), AstExprRef::Call(_))
      && !self.is_expr_mult_ret(node_ptr)
    {
      self.compile_expr(node, target, true);
      return false;
    }

    match node.as_expr_ref() {
      AstExprRef::Call(expr) => {
        let _rs = self.reg_scope_top(target as u32);
        self.compile_expr_call(expr, target, 0, true, true);
        return true;
      }
      AstExprRef::Varargs(expr) => {
        let _rs = self.reg_scope_top(target as u32);
        self.compile_expr_varargs(expr, target, 0, true);
        return true;
      }
      _ => {}
    }

    self.compile_expr(node, target, true);
    false
  }

  /// 对应 cpp `compileCompareJump`（cpp/Compiler/src/Compiler.cpp:1970 附近比较
  /// 分发路径）：把比较运算编译为单条条件跳转，返回待 patch 的跳转 pc——调用方
  /// 须在同一函数体内以 `patch_jump` 回填，悬空未 patch 会产出非法字节码。
  /// `expr` 为比较运算分发进来的存活节点（op 为比较类），`left`/`right` 子指针
  /// 只读消费。
  pub(crate) fn compile_compare_jump(&mut self, expr: &AstExprBinary, not_: bool) -> usize {
    {
      let expr_ref = expr;
      // left/right 已句柄化；本函数全程走既有裸指针编译 API（swap 交换语义不变），
      // 入口处经 as_ptr 折回，子指针存活由句柄非空契约保证。
      let mut left = expr_ref.left.as_ptr();
      let mut right = expr_ref.right.as_ptr();
      let is_eq =
        expr_ref.op == AstExprBinaryOp::CompareEq || expr_ref.op == AstExprBinaryOp::CompareNe;

      let mut operand_is_constant = self.is_constant(right);
      if is_eq && !operand_is_constant {
        operand_is_constant = self.is_constant(left);
        if operand_is_constant {
          swap(&mut left, &mut right);
        }
      }

      if operand_is_constant && (self.is_constant_vector(right) || self.is_constant_integer(right))
      {
        operand_is_constant = false;
      }

      let mut rs = self.reg_scope();
      let rl = self.compile_expr_auto(left, &mut rs);

      if is_eq && operand_is_constant {
        let cv = self.get_constant(right);
        LUAU_ASSERT!(!cv.is_unknown());

        let (opc, cid_val) = match cv {
          Constant::Nil => (LuauOpcode::LOP_JUMPXEQKNIL, 0),
          Constant::Boolean(b) => (LuauOpcode::LOP_JUMPXEQKB, b as i32),
          // 门面解引用：right 子指针由 parser 接线指向存活 AstExpr 节点（expr 为调用方
          // 持有的 arena 树内节点），get_constant_index 只读消费其常量表项。
          Constant::Number(_) => (
            LuauOpcode::LOP_JUMPXEQKN,
            self.get_constant_index(ast_slot_ref(right).expect("right 为 parser 接线的存活子指针")),
          ),
          Constant::Str(_) => (
            LuauOpcode::LOP_JUMPXEQKS,
            self.get_constant_index(ast_slot_ref(right).expect("right 为 parser 接线的存活子指针")),
          ),
          _ => {
            LUAU_ASSERT!(false);
            (LuauOpcode::LOP_NOP, 0)
          }
        };

        self.check_constant(cid_val, &expr_ref.base.base.location);

        let jump_label = self.bc().emit_label();
        let flip = if (expr_ref.op == AstExprBinaryOp::CompareEq) == not_ {
          K_GETIMPORT_FLAG
        } else {
          0
        };

        self.emit_ad_aux(opc, rl, 0, (cid_val as u32) | flip);

        jump_label
      } else {
        let opc = self.get_jump_op_compare(expr_ref.op, not_);
        let rr = self.compile_expr_auto(right, &mut rs);
        let jump_label = self.bc().emit_label();

        if expr_ref.op == AstExprBinaryOp::CompareGt || expr_ref.op == AstExprBinaryOp::CompareGe {
          self.emit_ad_aux(opc, rr, 0, rl as u32);
        } else {
          self.emit_ad_aux(opc, rl, 0, rr as u32);
        }
        jump_label
      }
    }
  }

  /// cpp `target` 出参寄存器指针（可空）Option 化：只读快照语义——本函数与
  /// 全部被调链仅读取该寄存器编号，从不写回，故由 `*const u8` 收为按值 Option。
  /// cpp `jumpPatchList` 出参改返回值：本函数只向列表追加新跳变标签、由调用方
  /// 接收后统一 patch，消灭 `&mut Vec` 累加器出参。
  ///
  /// 分支条件求值并生成跳转列表（cpp `compileConditionValue` 语义）：`node` 为
  /// 分支条件位置的 `AstExpr`（constants 键与子表达式下钻均经 parser 接线的 arena
  /// 指针，编译期内存活、只读）；`target` 若为 `Some` 须是已分配寄存器编号——
  /// 布尔值写入该槽。返回值为待逐条 patch 的跳转 pc 列表。
  pub(crate) fn compile_condition_value(
    &mut self,
    node: *mut AstExpr,
    target: Option<u8>,
    only_truth: bool,
  ) -> Vec<usize> {
    if let Some(cv) = self.constants.find(&node.into())
      && !cv.is_unknown()
    {
      if cv.is_truthful() == only_truth {
        if let Some(target) = target {
          // Safety: node 及其递归子指针（expr.left/right/expr.expr）均为 parser 接线
          // 的非空 arena 节点、整个编译期内存活（compile_or_throw 持有 parse arena 至
          // 返回）；target 是调用方条件分支已分配好的目标寄存器编号。
          self.compile_expr(unsafe { &mut *node }, target, true);
        }
        let label = self.bc().emit_label();
        self.bc_mut().emit_ad(LuauOpcode::LOP_JUMP, 0, 0);
        return vec![label];
      }
      return Vec::new();
    }

    // 门面判型+下转：node 指向 arena 内存活的 AstExpr（parser 接线、编译期存活），
    // 命中即动态类型为 AstExprBinary；本函数对 AST 只读，共享借用与 self 无重叠。
    if let Some(expr) = ast_slot_try_as::<AstExprBinary, _>(node) {
      match expr.op {
        AstExprBinaryOp::And | AstExprBinaryOp::Or => {
          if only_truth == (expr.op == AstExprBinaryOp::And) {
            // left/right 已句柄化；compile_condition_value 为既有裸指针 API，经 as_ptr 桥接。
            let else_jump = self.compile_condition_value(expr.left.as_ptr(), None, !only_truth);
            let skip_jump = self.compile_condition_value(expr.right.as_ptr(), target, only_truth);
            let else_label = self.bc().emit_label();
            // 门面解引用：node 为存活 arena 节点，仅取基类供 patch_jumps 定位报错。
            self.patch_jumps(
              &ast_slot_ref(node)
                .expect("node 为 parser 接线的存活 arena 节点")
                .base,
              &else_jump,
              else_label,
            );
            return skip_jump;
          }
          // 同上：既有裸指针 API 经 as_ptr 桥接。
          let mut skip_jump = self.compile_condition_value(expr.left.as_ptr(), target, only_truth);
          skip_jump.extend(self.compile_condition_value(expr.right.as_ptr(), target, only_truth));
          return skip_jump;
        }
        op if is_compare_op(op) => {
          if let Some(target_reg) = target {
            self.bc_mut().emit_abc(
              LuauOpcode::LOP_LOADB,
              target_reg,
              if only_truth { 1 } else { 0 },
              0,
            );
          }
          // expr 由上方 RTTI 命中得到，与 node 基址重合（repr(C) 首字段）；
          // target_reg 来自本帧 Some(target)。
          let jump_label = self.compile_compare_jump(expr, !only_truth);
          return vec![jump_label];
        }
        _ => {}
      }
    }

    // 门面判型+下转：node 为 parser 接线的存活 arena 节点（同上），命中即动态类型
    // 为 AstExprUnary，读取 op/expr 字段合法；不写字节码以外内容，与 self 借用无重叠。
    if let Some(expr) = ast_slot_try_as::<AstExprUnary, _>(node)
      && target.is_none()
      && expr.op == AstExprUnaryOp::Not
    {
      // expr 已句柄化；compile_condition_value 为既有裸指针 API，经 as_ptr 桥接。
      return self.compile_condition_value(expr.expr.as_ptr(), target, !only_truth);
    }

    // 门面判型+下转：node 存活同上，命中即动态类型为 AstExprGroup，基址重合保证下转合法。
    if let Some(expr) = ast_slot_try_as::<AstExprGroup, _>(node) {
      // expr 已句柄化；compile_condition_value 仍为既有裸指针 API，经 as_ptr 桥接（只读传参）。
      return self.compile_condition_value(expr.expr.as_ptr(), target, only_truth);
    }

    let mut rs = self.reg_scope();
    let reg = if let Some(target) = target {
      // Safety: node 为 parser 接线的 arena 存活节点，target 为 Some 分支的已分配
      // 目标寄存器编号。
      self.compile_expr(unsafe { &mut *node }, target, true);
      target
    } else {
      self.compile_expr_auto(node, &mut rs)
    };

    let label = self.bc().emit_label();
    self.bc_mut().emit_ad(
      if only_truth {
        LuauOpcode::LOP_JUMPIF
      } else {
        LuauOpcode::LOP_JUMPIFNOT
      },
      reg,
      0,
    );
    vec![label]
  }

  pub(crate) fn get_binary_op_arith(&self, op: AstExprBinaryOp, k: bool) -> LuauOpcode {
    if let Some([normal, kvariant]) = ARITH_OPS.get(op as usize) {
      return if k { *kvariant } else { *normal };
    }
    LUAU_ASSERT!(false);
    LuauOpcode::LOP_NOP
  }

  pub(crate) fn get_jump_op_compare(&self, op: AstExprBinaryOp, not_: bool) -> LuauOpcode {
    match (op, not_) {
      (AstExprBinaryOp::CompareNe, true) | (AstExprBinaryOp::CompareEq, false) => {
        LuauOpcode::LOP_JUMPIFEQ
      }
      (AstExprBinaryOp::CompareNe, false) | (AstExprBinaryOp::CompareEq, true) => {
        LuauOpcode::LOP_JUMPIFNOTEQ
      }
      (AstExprBinaryOp::CompareLt | AstExprBinaryOp::CompareGt, true) => {
        LuauOpcode::LOP_JUMPIFNOTLT
      }
      (AstExprBinaryOp::CompareLt | AstExprBinaryOp::CompareGt, false) => LuauOpcode::LOP_JUMPIFLT,
      (AstExprBinaryOp::CompareLe | AstExprBinaryOp::CompareGe, true) => {
        LuauOpcode::LOP_JUMPIFNOTLE
      }
      (AstExprBinaryOp::CompareLe | AstExprBinaryOp::CompareGe, false) => LuauOpcode::LOP_JUMPIFLE,
      _ => {
        LUAU_ASSERT!(false);
        LuauOpcode::LOP_NOP
      }
    }
  }

  pub(crate) const fn get_unary_op(&self, op: AstExprUnaryOp) -> LuauOpcode {
    match op {
      AstExprUnaryOp::Not => LuauOpcode::LOP_NOT,
      AstExprUnaryOp::Minus => LuauOpcode::LOP_MINUS,
      AstExprUnaryOp::Len => LuauOpcode::LOP_LENGTH,
    }
  }

  pub(crate) fn is_condition_fast(&mut self, node: *mut AstExpr) -> bool {
    // null 先行早退；constants.find 仅以指针地址为键，不解引用。
    if node.is_null() {
      return false;
    }

    {
      let cv = self.constants.find(&node.into());

      if cv.is_some_and(|constant| !constant.is_unknown()) {
        return true;
      }

      // 门面判型+下转：node 非空且为 arena 存活 AstExpr（递归沿 AST 树下行）；
      // 命中即动态类型 AstExprBinary、repr(C) 首字段一致。And/Or 或任一比较
      // 运算（比较族单点在 is_compare_op）均可快速条件化。
      if let Some(binary) = ast_slot_try_as::<AstExprBinary, _>(node) {
        return matches!(binary.op, AstExprBinaryOp::And | AstExprBinaryOp::Or)
          || is_compare_op(binary.op);
      }

      // 门面判型+下转：同上，命中即 AstExprGroup；group.expr 已句柄化恒非空，
      // 递归沿 AST 树下行、必终止；is_condition_fast 仍为既有裸指针 API，经 as_ptr 桥接。
      if let Some(group) = ast_slot_try_as::<AstExprGroup, _>(node) {
        return self.is_condition_fast(group.expr.as_ptr());
      }

      false
    }
  }

  /// C++ `isExprMultRet`：调用（非内建或多返回值内建）或 varargs 视为多返回值。
  pub(crate) fn is_expr_mult_ret(&self, node: *mut AstExpr) -> bool {
    // 门面判型+下转：null 输入返回 None，命中说明 node 经 class index 校验确为
    // AstExprCall（repr(C) 首字段一致），字段可安全读取；node 来自 parser arena、
    // 编译期只读存活，后续 expr.func/expr 地址查询都只依赖这一存活性。
    let Some(expr) = ast_slot_try_as::<AstExprCall, _>(node) else {
      // 门面判型（只读 class index，null 折叠 false）。
      return ast_slot_is::<AstExprVarargs, _>(node);
    };

    if self.options.optimization_level <= 1 {
      return true;
    }

    if self.is_constant(node) {
      return false;
    }

    if self.options.optimization_level >= 2
      && let Some(bfid) = self.builtins.find(&Node::from_ref(expr))
      && *bfid != LuauBuiltinFunction::LBF_NONE as i32
    {
      return get_builtin_info(*bfid).results != 1;
    }

    // get_function_expr 未命中（None）即无建档函数，fi 归 None（同 cpp null 哨兵）
    let fi = self
      .get_function_expr(expr.func)
      .and_then(|func| self.functions.find(&func));

    !fi.is_some_and(|fi| fi.returns_one)
  }

  /// C++ `unrollConcats`：栈顶为 Concat 时展开为左右操作数，直到栈顶不可展开。
  pub(crate) fn unroll_concats(&self, args: &mut Vec<*mut AstExpr>) {
    while let Some(&back) = args.last() {
      // 对应 C++ `args.back()->as<AstExprBinary>()`——带 CHECKED 的 RTTI 下转，
      // 节点不是 AstExprBinary 时返回 null。旧模型用
      // `as_expr() as *mut AstExprBinary` 盲转，把任意节点（如
      // `a..b..c` 尾部的字符串）当作 Binary 并解引用其垃圾
      // `op`/`left`/`right` → SIGSEGV。
      // 门面契约：back 是 args 栈顶元素，来自 parser 产出的存活表达式指针
      // （concat 展开自左结合二叉链）；checked 下转未命中返回 None，
      // 命中即动态类型为 AstExprBinary，读 op/left/right 合法。
      let Some(be) = ast_slot_try_as::<AstExprBinary, _>(back) else {
        break;
      };

      if be.op != AstExprBinaryOp::Concat {
        break;
      }

      // left/right 已句柄化恒非空；args 行走链为既有裸指针 API，经 as_ptr 桥接。
      args.pop();
      args.push(be.left.as_ptr());
      args.push(be.right.as_ptr());
    }
  }

  /// cpp Compiler.cpp 各调用点的 `for (size_t i = 0; i < targetCount; i++) emit(MOVE, target+i, regs+i)`：
  /// 把从 `regs` 起的 `count` 个连续寄存器逐个搬回 `target`。寄存器号用 wrapping_add，
  /// 与 C++ `unsigned char` 回绕语义一致（寄存器窗口由 alloc_reg 保证在界内，正常输入不回绕）。
  pub(crate) fn fill_regs_move(&mut self, target: u8, regs: u8, count: u8) {
    for i in 0..count {
      self.bc_mut().emit_abc(
        LuauOpcode::LOP_MOVE,
        target.wrapping_add(i),
        regs.wrapping_add(i),
        0,
      );
    }
  }

  /// cpp `for (i = from; i < to; i++) emit(LOADNIL, target+i)`：把寄存器窗口 `[from, to)` 清空为 nil。
  /// 回绕语义同 [`fill_regs_move`](Self::fill_regs_move)。
  pub(crate) fn fill_regs_nil(&mut self, target: u8, from: u8, to: u8) {
    for i in from..to {
      self
        .bc_mut()
        .emit_abc(LuauOpcode::LOP_LOADNIL, target.wrapping_add(i), 0, 0);
    }
  }
}

/// 算术二元 opcode 编译期查找表：索引为 `AstExprBinaryOp` 判别值（`Add`=0..`Pow`=6），
/// 两列分别是非 K / K（右操作数为常量）变体；表外 opcode 走原断言路径返回 `NOP`。
const ARITH_OPS: [[LuauOpcode; 2]; 7] = [
  [LuauOpcode::LOP_ADD, LuauOpcode::LOP_ADDK],
  [LuauOpcode::LOP_SUB, LuauOpcode::LOP_SUBK],
  [LuauOpcode::LOP_MUL, LuauOpcode::LOP_MULK],
  [LuauOpcode::LOP_DIV, LuauOpcode::LOP_DIVK],
  [LuauOpcode::LOP_IDIV, LuauOpcode::LOP_IDIVK],
  [LuauOpcode::LOP_MOD, LuauOpcode::LOP_MODK],
  [LuauOpcode::LOP_POW, LuauOpcode::LOP_POWK],
];
