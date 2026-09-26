//! `Compiler` 函数与闭包：`compile_function`、共享闭包判定、upvalue 解析与
//! 内联调用展开（对照 cpp `Compiler.cpp` 的 compileFunction / inlined-call 段）。
use alloc::vec::Vec;
use core::{cmp::min, mem::take, ptr::from_mut};

use ulua_ast::{
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{
    ast_attr::AstAttrType::DebugNoinline, ast_expr::AstExpr, ast_expr_call::AstExprCall,
    ast_expr_function::AstExprFunction, ast_expr_group::AstExprGroup,
    ast_expr_index_name::AstExprIndexName, ast_expr_instantiate::AstExprInstantiate,
    ast_expr_local::AstExprLocal, ast_expr_type_assertion::AstExprTypeAssertion,
    ast_expr_varargs::AstExprVarargs, ast_local::AstLocal, ast_stat::AstStat,
    ast_stat_block::AstStatBlock, ast_stat_class::AstStatClass, ast_stat_function::AstStatFunction,
    ast_stat_return::AstStatReturn,
  },
  rtti::{ast_node_try_as, ast_node_try_as_ptr, ast_node_try_as_ptr_mut},
};
use ulua_common::{
  enums::{
    luau_bytecode_type::LuauBytecodeType, luau_opcode::LuauOpcode, luau_proto_flag::LuauProtoFlag,
  },
  fflag,
  fflag::DebugLuauUserDefinedClasses,
  macros::{
    luau_assert::LUAU_ASSERT, luau_timetrace_argument::LUAU_TIMETRACE_ARGUMENT,
    luau_timetrace_scope::LUAU_TIMETRACE_SCOPE,
  },
};

use crate::{
  functions::{
    always_terminates::always_terminates,
    analyze_builtins::analyze_builtins,
    ast_slot_ref::ast_slot_try_as,
    compute_cost::compute_cost,
    cost_model::model_cost,
    sref_compiler::sref_ast_name,
    undo_changes_constant_folding::{undo_changes_expr, undo_changes_local},
  },
  records::{
    compile_error::{CompileError, ERR_EXCEEDED_JUMP_DISTANCE_LIMIT, REMARK_INLINE_RECURSIVE},
    compiler::{Compiler, K_COST_PERCENT_SCALE, K_DEFAULT_ALLOC_PC, K_INVALID_REG},
    const_upvalue_visitor::ConstUpvalueVisitor,
    constant::Constant,
    inline_arg::InlineArg,
    inline_frame::InlineFrame,
    node::Node,
    return_visitor::ReturnVisitor,
  },
};
/// cpp `BytecodeBuilder.h:16` `kMaxInstructionCount`（10 亿，非 100 万）
const K_MAX_INSTRUCTION_COUNT: usize = 1_000_000_000;

impl Compiler {
  /// `func_ref` 由调用方 `&mut` 交接证明存活且独占（主 chunk 为 compile_or_throw
  /// 栈上节点、闭包为 arena 节点）；`body`/`args`/`self_` 子指针为 parser 接线
  /// 存活对象，`functions` 表以节点地址为键（由借用还原）。
  /// 元组返回 `(函数 id, protoflags)` 替代 cpp 的出参形态。
  pub fn compile_function(
    &mut self,
    func_ref: &mut AstExprFunction,
    mut protoflags: u8,
  ) -> (u32, u8) {
    LUAU_TIMETRACE_SCOPE!("Compiler::compileFunction", "Compiler");

    // 地址快照：仅作 functions/current_function/function_types 的键，
    // 不重建引用；由本借用证明存活的节点升格为句柄。
    let func_node = Node::from_mut(&mut *func_ref);
    // body 槽位句柄为 Copy 的 arena 地址载体：先复制到局部，再从其 get_mut 交出
    // 块的可变借用（对应 cpp `AstStatBlock* body = func->body;`）。借用只锁住该
    // 局部句柄，func_ref 其余字段的读写窗口不被块借用串行化；块与函数头是互不
    // 重叠的两块 arena 内存，独占性由上面的 `&mut func_ref` 交付证明。
    let mut body_handle = func_ref.body;
    let body = body_handle.get_mut();

    if !func_ref.debugname.is_null() {
      LUAU_TIMETRACE_ARGUMENT!("name", func_ref.debugname.value);
    }

    LUAU_ASSERT!(!self.functions.contains(&func_node));
    LUAU_ASSERT!(
      self.reg_top == 0
        && self.stack_size == 0
        && self.local_stack.is_empty()
        && self.upvals.is_empty()
    );
    if fflag::LuauExportValueSyntax.get() {
      self.current_function = Some(func_node);
    }

    let _rs = self.reg_scope();
    let self_ = if func_ref.self_.is_some() { 1 } else { 0 };
    let fid = self
      .bc_mut()
      .begin_function((self_ + func_ref.args.len()) as u8, func_ref.vararg);

    self.set_debug_line_ast_node(&func_ref.base.base);

    if func_ref.vararg {
      self.bc_mut().emit_abc(
        LuauOpcode::LOP_PREPVARARGS,
        (self_ + func_ref.args.len()) as u8,
        0,
        0,
      );
    }

    let args = self.alloc_reg(&func_ref.base.base, (self_ + func_ref.args.len()) as u32);
    // cpp 传 kDefaultAllocPc（哨兵 ~0u）：push_local 内部落为 debugpc，
    // 传 0 会把参数的 allocpc 错误固定为 0，破坏 local type info
    // self_ 槽已句柄化（OptNode）：判空由 Option 兑现，`get` 即存活共享引用。
    if let Some(self_local) = func_ref.self_.get() {
      self.push_local(self_local, args, K_DEFAULT_ALLOC_PC);
    }
    // args 已句柄化（Nodes）：iter 直接交出 &AstLocal，非空由构造端证明。
    for (i, arg_local) in func_ref.args.iter().enumerate() {
      self.push_local(arg_local, args + self_ as u8 + i as u8, K_DEFAULT_ALLOC_PC);
    }

    self.arg_count = self.local_stack.len();
    let mut terminates_early = false;
    self.current_function = Some(func_node);

    if DebugLuauUserDefinedClasses.get() && self.at_top_level() {
      self.preallocate_hoisted_classes(Some(body));
    }

    for body_stat in body.body.iter_nodes() {
      self.compile_stat(body_stat.as_ptr());
      if always_terminates(&self.constants, body_stat.as_ptr()) {
        terminates_early = true;
        break;
      }
    }

    if fflag::LuauExportValueSyntax.get() {
      self.set_debug_line_end(&body.base.base);
      // cpp:525-531：`!exports.isEmpty() && atTopLevel()`；
      // cpp:529-531 的 `LuauExportedTypesParticipateInScc && exports.hasTypeExports`
      // 分支未移植（见 compile_export_table 注释），旗标缺失时该分支在上游同样不触发。
      if !self.exports_is_empty() && self.at_top_level() {
        self.compile_export_table();
      } else if !terminates_early {
        self.close_locals(0);
        self.bc_mut().emit_abc(LuauOpcode::LOP_RETURN, 0, 1, 0);
      }
    } else if !terminates_early {
      self.set_debug_line_end(&body.base.base);
      self.close_locals(0);
      self.bc_mut().emit_abc(LuauOpcode::LOP_RETURN, 0, 1, 0);
    }

    if self.options.optimization_level >= 1 && self.options.debug_level >= 2 {
      self.gather_const_upvals(func_ref);
    }

    self
      .bc_mut()
      .set_debug_function_line_defined(func_ref.base.base.location.begin.line as i32 + 1);

    if self.options.debug_level >= 1 && !func_ref.debugname.is_null() {
      self
        .bc_mut()
        .set_debug_function_name(sref_ast_name(func_ref.debugname));
    }

    // 上值名单与上值类型信息两轮登记合并为单次遍历：take 出走借用使
    // `bc_mut()` 的 `&mut self` 与上值迭代不重叠，收尾归位保持容量复用。
    if self.options.debug_level >= 2 || self.options.type_info_level >= 1 {
      let upvals = take(&mut self.upvals);
      for &upval in &upvals {
        if self.options.debug_level >= 2 {
          // upval 为 get_upval 登记的存活节点句柄（parser arena 或
          // export_table_local 字段地址），仅读 name。
          let name = upval.borrow().name;
          self.bc_mut().push_debug_upval(sref_ast_name(name));
        }

        if self.options.type_info_level >= 1 {
          let ty = self
            .local_types
            .find(&upval)
            .copied()
            .unwrap_or(LuauBytecodeType::LBC_TYPE_ANY);
          self.bc_mut().push_upval_type_info(ty);
        }
      }
      self.upvals = upvals;
    }

    if self.options.optimization_level >= 1 {
      self.bc_mut().fold_jumps();
    }
    // cpp Compiler.cpp:586-589：trampoline 展开后仍超 JUMPX 射程须报错，
    // 否则 24 位偏移静默截断产生坏字节码
    let has_long_jump_error = self.bc_mut().expand_jumps();
    if fflag::LuauCompileExpandLimit.get() && has_long_jump_error {
      CompileError::raise(
        &func_ref.base.base.location,
        format_args!("{ERR_EXCEEDED_JUMP_DISTANCE_LIMIT}"),
      );
    }
    self.pop_locals(0);

    if self.bc_mut().get_instruction_count() > K_MAX_INSTRUCTION_COUNT {
      CompileError::raise(
        &func_ref.base.base.location,
        format_args!(
          "Exceeded function instruction limit; split the function into parts to compile"
        ),
      );
    }

    if let Some(func_type) = self.function_types.get(&func_node).cloned() {
      self.bc_mut().set_function_type_info(func_type);
    }

    if func_ref.function_depth == 0 && !self.has_loops {
      protoflags |= LuauProtoFlag::LPF_NATIVE_COLD;
    }
    if func_ref.has_native_attribute() {
      protoflags |= LuauProtoFlag::LPF_NATIVE_FUNCTION;
    }

    // cpp:607-608：主 chunk 携带 __EXP 上值/导出表时置位，供 VM 侧 lua_usesexport 读取
    // （ulua-vm/src/functions/lua_usesexport.rs，CLI require 链路据此建循环占位表）。
    // cpp:610-611 的 `LuauExportedTypesParticipateInScc && exports.hasTypeExports`
    // 分支未移植：旗标与 hasTypeExports 跟踪都不存在，上游关闭旗标时同样不置位。
    if fflag::LuauExportValueSyntax.get()
      && !self.exports_is_empty()
      && func_ref.function_depth == 0
    {
      protoflags |= LuauProtoFlag::LPF_USES_EXPORT;
    }

    // cpp:613：与上游同条件；hasMultiRet 由 compile_stat_return 置位、本函数收尾复位
    let is_inlinable = !self.has_multi_ret && !self.getfenv_used && !self.setfenv_used;
    let mut cost_model = 0u64;
    if fflag::LuauEmitCallFeedback.get() && is_inlinable && self.upvals.is_empty() {
      protoflags |= LuauProtoFlag::LPF_INLINABLE;
      // C++ (Compiler.cpp:618) 传 `builtins` 成员（O>=1 恒有 analyzeBuiltins
      // 结果），而非 O2 才赋值的 builtinsFold 别名指针
      let arg_nodes: Vec<Node<_>> = func_ref.args.iter_nodes().map(|n| Node::from(*n)).collect();
      cost_model = model_cost(
        &mut body.base.base,
        &arg_nodes,
        &self.builtins,
        &self.constants,
      );
    }

    // 先取 Copy 状态位，避免 bc_mut 的可变借用与 self 字段读取同表达式重叠
    let max_stack_size = self.stack_size as u8;
    let num_upvalues = self.upvals.len() as u8;
    self
      .bc_mut()
      .end_function(max_stack_size, num_upvalues, protoflags, cost_model);

    {
      let f = self.functions.get_or_insert(func_node);
      f.id = fid;
      f.upvals = self.upvals.clone();
    }

    if self.options.optimization_level >= 2
      && !func_ref.vararg
      && func_ref.self_.is_none()
      && !self.getfenv_used
      && !self.setfenv_used
    {
      // C++: `f.costModel = costModel == 0 ? modelCost(...) : costModel;`
      // 优先复用 LPF_INLINABLE 路径已算好的模型，避免重复遍历 AST；
      // 同 cpp:618 传 `builtins` 成员而非 builtinsFold
      let arg_nodes: Vec<Node<_>> = func_ref.args.iter_nodes().map(|n| Node::from(*n)).collect();
      let cost_model = if cost_model == 0 {
        model_cost(
          &mut body.base.base,
          &arg_nodes,
          &self.builtins,
          &self.constants,
        )
      } else {
        cost_model
      };
      let returns_one =
        if always_terminates(&self.constants, func_ref.body.as_ptr().cast::<AstStat>()) {
          let mut rv = ReturnVisitor::new(self);
          ast_stat_block_visit(body, &mut rv);
          Some(rv.returns_one)
        } else {
          None
        };

      let f = self.functions.get_or_insert(func_node);
      // C++: `f.canInline = !(DebugLuauNoInline && func->hasAttribute(DebugNoinline))`.
      f.can_inline =
        !(fflag::DebugLuauNoInline.get() && !func_ref.get_attribute(DebugNoinline).is_null());
      f.stack_size = self.stack_size;
      f.cost_model = cost_model;
      if let Some(returns_one) = returns_one {
        f.returns_one = returns_one;
      }
    }

    self.upvals.clear();
    self.stack_size = 0;
    self.arg_count = 0;
    self.has_loops = false;
    // cpp:668：per-function 状态复位（LPF_INLINABLE 判定用）
    self.has_multi_ret = false;
    self.current_function = None;
    (fid, protoflags)
  }

  /// 执行内联：求值实参、建帧、对 body 做内置分析/常量折叠后逐语句编译，最后回滚状态。
  ///
  /// `expr`/`func` 须为 `try_compile_inlined_call` 判定可内联后传入的配对节点，且
  /// `func` 确为 `expr` 的被调体。`target` 起 `target_count` 寄存器须已分配——内联体
  /// 直接占用该区间，失配将覆写宿主函数活跃值（错误字节码，非内存不安全）。
  pub(crate) fn compile_inlined_call(
    &mut self,
    expr: &AstExprCall,
    func: &AstExprFunction,
    target: u8,
    target_count: u8,
  ) {
    // inline_frames 以 arena 地址为帧键，引用实参由 Node 句柄承接
    let func_node = Node::from_ref(func);
    // Safety: func.body 是 parser 在 arena 接线并保证非空的函数体根节点，比编译存活；
    // 本函数只持 `&func`（cpp 侧同源 const_cast），折叠/编译对 body 子树的独占交接
    // 由该 arena 存活契约与「同一时刻仅一个借用窗口」保证。
    let body = unsafe { &mut *func.body.as_ptr() };

    let _rs = self.reg_scope();
    let old_locals = self.local_stack.len();

    let func_args_size = func.args.len();
    let expr_args_size = expr.args.size;
    // AstArray::as_slice 内部已守 null（size == 0 时 data 可能为 null）
    let func_args = func.args.as_slice();
    let expr_args = expr.args.as_slice();

    let mut args: Vec<InlineArg> = Vec::with_capacity(func_args_size);

    // 求值全部实参；注意常量实参不发射代码（依赖常量折叠）
    // 并注意此处不改动编译器状态（变量寄存器/值）——统一延后到下方第二个循环，以正确处理嵌套调用
    for (i, &var) in func_args.iter().enumerate() {
      // 实参缺失（调用点少给）= None，取代 cpp 的 null 哨兵
      let arg: Option<*mut AstExpr> = expr_args.get(i).copied();

      if i + 1 == expr_args_size
        && func_args_size > expr_args_size
        && arg.is_some_and(|arg| self.is_expr_mult_ret(arg))
      {
        // 末位实参可返回多值时，需把全部返回值算进剩余的形参寄存器
        let tail: u32 = (func_args_size - expr_args_size) as u32 + 1;
        let reg = self.alloc_reg(&expr.base.base, tail);
        let allocpc = self.bc().get_debug_pc();

        // Safety: arg 为 arena 存活子表达式指针，指针版 RTTI 门面判型后给出的
        // 'static 引用指向 parser 接线节点，本次分支内独占使用、随语句结束丢弃
        if let Some(call) =
          arg.and_then(|arg| unsafe { ast_node_try_as_ptr_mut::<AstExprCall>(arg) })
        {
          self.compile_expr_call(from_mut(call), reg, tail as u8, true, false);
        } else if let Some(va) = arg.and_then(ast_slot_try_as::<AstExprVarargs, _>) {
          self.compile_expr_varargs(va, reg, tail as u8, false);
        } else {
          LUAU_ASSERT!(false, "Unexpected expression type");
        }

        // 其余全部形参已完成分配与赋值
        for (offset, &local) in func_args[i..].iter().enumerate() {
          args.push(InlineArg {
            local: local.into(),
            reg: reg + offset as u8,
            value: Constant::Unknown,
            allocpc,
            init: None,
          });
        }
        break;
      } else if self
        .variables
        .find(&var.into())
        .is_some_and(|vv| vv.written)
      {
        // 实参被改写时必须新分配寄存器，即便它是常量
        let reg = self.alloc_reg(&expr.base.base, 1u32);
        let allocpc = self.bc().get_debug_pc();
        if let Some(arg) = arg {
          // Safety: arg 为 arena 存活子表达式指针，独占借用于此处交接给求值路径后即收回
          self.compile_expr(unsafe { &mut *arg }, reg, true);
        } else {
          self.bc_mut().emit_abc(LuauOpcode::LOP_LOADNIL, reg, 0, 0);
        }
        args.push(InlineArg {
          local: var.into(),
          reg,
          value: Constant::Unknown,
          allocpc,
          init: None,
        });
      } else if arg.is_none() {
        // 实参未被改写，可把值直接折叠进需要它的表达式
        args.push(InlineArg {
          local: var.into(),
          reg: K_INVALID_REG,
          value: Constant::Nil,
          allocpc: K_DEFAULT_ALLOC_PC,
          init: None,
        });
      } else if let Some(cv) = arg.and_then(|arg| {
        self
          .constants
          .find(&arg.into())
          .filter(|cv| !cv.is_unknown())
      }) {
        // 实参未被改写，可把值直接折叠进需要它的表达式
        args.push(InlineArg {
          local: var.into(),
          reg: K_INVALID_REG,
          value: *cv,
          allocpc: K_DEFAULT_ALLOC_PC,
          init: None,
        });
      } else if let Some(arg) = arg {
        let le = self.get_expr_local(arg);
        // 一次性取出 written/init，避免与 get_expr_local_reg 的可变借用冲突；
        // le 为 arena 存活 AstExprLocal 句柄，仅读其 .local 键值作 variables 表查询
        let lv = le.map(|le| {
          self
            .variables
            .find(&le.borrow().local.into())
            .map(|v| (v.written, v.init))
        });

        // 实参是未被改写的 local 时，直接复用其现有寄存器
        let reg = le.map_or(-1, |le| self.get_expr_local_reg(le.cast::<AstExpr>()));
        if reg >= 0 && lv.flatten().is_none_or(|(written, _)| !written) {
          args.push(InlineArg {
            local: var.into(),
            reg: reg as u8,
            value: Constant::Unknown,
            allocpc: K_DEFAULT_ALLOC_PC,
            // cpp `lv ? lv->init : nullptr` → Option 链（无 local 记录或无初值皆 None）
            init: lv.flatten().and_then(|(_, init)| init),
          });
        } else {
          let temp = self.alloc_reg(&expr.base.base, 1u32);
          let allocpc = self.bc().get_debug_pc();
          // Safety: arg 为 arena 存活子表达式指针，独占借用于此处交接给求值路径后即收回
          self.compile_expr(unsafe { &mut *arg }, temp, true);
          args.push(InlineArg {
            local: var.into(),
            reg: temp,
            value: Constant::Unknown,
            allocpc,
            init: Some(arg.into()),
          });
        }
      }
    }

    // 多余实参仍需求值以保留副作用
    if let Some(extra) = expr_args.get(func_args_size..) {
      for &side in extra {
        self.compile_expr_side(side);
      }
    }

    // 把已求值的实参统一应用到编译器状态
    // 注：local 调试信息用当前 startpc，尽管部分实参更早算出；与 compileStatLocal 同理
    for arg in &args {
      if arg.value.is_unknown() {
        // arg.local 为 arena 存活 AstLocal 句柄（存活契约见 `Node::borrow`），
        // push_local 仅读节点头部字段并写编译器状态
        self.push_local(arg.local.borrow(), arg.reg, arg.allocpc);
        if let Some(init) = arg.init
          && let Some(lv) = self.variables.find_mut(&arg.local)
        {
          lv.init = Some(init);
        }
      } else {
        *self.locstants.get_or_insert(arg.local) = arg.value;
      }
    }

    self.inline_frames.push(InlineFrame {
      func: func_node,
      local_offset: old_locals,
      target,
      target_count,
      return_jumps: Vec::new(),
    });

    // 返回值形态收口：analyze_builtins 自带结果 map，整体内建表无需再借
    // `&raw mut self.inline_builtins` 拆借用（旧裸指针窗口消除）。
    // `&mut body.base.base` 为函数体根的独占视图，analyze_builtins 对其只读遍历；
    // 调用前 inline_builtins 恒为空（上轮末尾 clear），整体赋值等价于清空后重建。
    self.inline_builtins = analyze_builtins(
      &self.globals,
      &self.variables,
      &self.options,
      &mut body.base.base,
      self.names(),
    );

    // 若发现新内建则应用之，同时记录改过的表达式以便日后回滚
    if !self.inline_builtins.is_empty() {
      // 字段解构拆借：直接迭代 inline_builtins 并写 builtins/backup，免去 collect 中转分配
      let Self {
        inline_builtins,
        builtins,
        inline_builtins_backup,
        ..
      } = self;
      for (&call_expr, &bfid) in inline_builtins.iter() {
        let builtin = *builtins.get_or_insert(call_expr);
        if bfid != builtin {
          *inline_builtins_backup.get_or_insert(call_expr) = builtin;
          *builtins.get_or_insert(call_expr) = bfid;
        }
      }
      inline_builtins.clear();
    }

    self.expr_changes.clear();
    self.local_changes.clear();

    self.fold_constants(&mut body.base.base, true);

    let mut terminates_early = false;
    for stat in body.body.iter_nodes() {
      self.compile_stat(stat.as_ptr());
      if always_terminates(&self.constants, stat.as_ptr()) {
        terminates_early = true;
        let curr_frame = self.inline_frames.last_mut().unwrap();
        if !curr_frame.return_jumps.is_empty() {
          let last_jump = *curr_frame.return_jumps.last().unwrap();
          // last_jump 取走后借用即尽；标签/回退用 bc()、bc_mut() 现取现还，
          // 末尾重取 last_mut 完成 pop，避免帧可变借用横跨 bytecode 调用。
          if last_jump == self.bc().emit_label() - 1 {
            self.bc_mut().undo_emit(LuauOpcode::LOP_JUMP);
            self.inline_frames.last_mut().unwrap().return_jumps.pop();
          }
        }
        break;
      }
    }

    if !terminates_early {
      self.fill_regs_nil(target, 0, target_count);
      self.close_locals(old_locals);
    }

    self.pop_locals(old_locals);
    let return_label = self.bc().emit_label();
    // 先把栈帧取出再补跳：cpp 用 `inlineFrames.back()` + 裸指针同时访问 self，
    // Rust 下 `patch_jumps(&mut self, ..)` 与 `&mut self.inline_frames` 借用互斥；
    // 取出帧后二者不再重叠，跳转表随后即随帧丢弃（patch 路径不读回）。
    let frame = self.inline_frames.pop().unwrap();
    self.patch_jumps(&expr.base.base, &frame.return_jumps, return_label);

    // 清理常量状态，供后续内联尝试
    for &local in func_args {
      if let Some(var) = self.locstants.find_mut(&local.into()) {
        *var = Constant::Unknown;
      }
      if let Some(lv) = self.variables.find_mut(&local.into()) {
        lv.init = None;
      }
    }

    if !self.inline_builtins_backup.is_empty() {
      let Self {
        inline_builtins_backup,
        builtins,
        ..
      } = self;
      for (&call_expr, &bfid) in inline_builtins_backup.iter() {
        *builtins.get_or_insert(call_expr) = bfid;
      }
      inline_builtins_backup.clear();
    }

    undo_changes_expr(&mut self.constants, &self.expr_changes);
    undo_changes_local(&mut self.locstants, &self.local_changes);
  }

  /// 内联判定：成本/寄存器压力/递归深度各项过关后才真正 `compile_inlined_call`。
  ///
  /// `func` 须为 `expr` 的被调体且已在 `functions` 表登记（阈值判定与递归深度
  /// 限制基于此地址键）；`target` 起 `target_count` 寄存器须已分配，`mult_ret`
  /// 须与调用方展开协议一致。返回 false 表示放弃内联——寄存器窗口回到调用前
  /// 水位由内部 RegScope 保证。
  pub(crate) fn try_compile_inlined_call(
    &mut self,
    expr: &AstExprCall,
    func: &AstExprFunction,
    target: u8,
    target_count: u8,
    mult_ret: bool,
    threshold_base: i32,
    threshold_max_boost: i32,
    depth_limit: i32,
  ) -> bool {
    // functions/inline_frames 均以 arena 地址为键，引用实参经 Node::from_ref 造键
    let func_node = Node::from_ref(func);
    let fi = self.functions.find(&func_node);
    LUAU_ASSERT!(fi.is_some());
    // Safety: 调用方仅在 functions.find 命中且 can_inline 为真时进入本路径，且编译期
    // 该表只增不删，故 unwrap_unchecked 的 Some 前提成立；fi 字段读取均为 Copy 取值。
    let fi = unsafe { fi.unwrap_unchecked() };
    let fi_stack_size = fi.stack_size;
    let fi_cost_model = fi.cost_model;
    let mut call_cost_model = fi_cost_model;

    if self.reg_top > K_MAX_INLINE_REG_TOP || fi_stack_size > K_MAX_INLINE_STACK_SIZE {
      self
        .bc_mut()
        .add_debug_remark(format_args!("inlining failed: high register pressure"));
      return false;
    }

    if self.inline_frames.len() as i32 >= depth_limit {
      self
        .bc_mut()
        .add_debug_remark(format_args!("inlining failed: too many inlined frames"));
      return false;
    }

    for frame in &self.inline_frames {
      if frame.func == func_node {
        self
          .bc_mut()
          .add_debug_remark(format_args!("{REMARK_INLINE_RECURSIVE}"));
        return false;
      }
    }

    if mult_ret {
      self.bc_mut().add_debug_remark(format_args!(
        "inlining failed: can't convert fixed returns to multret"
      ));
      return false;
    }

    let func_args_size = func.args.len();
    let expr_args_size = expr.args.size;

    // 为全部实参计算常量位向量，喂给代价模型
    let mut varc = [false; K_MAX_COST_ARGS];
    let mut has_constant = false;

    // AstArray::as_slice 已守 null（size == 0 时 data 可能为 null），无需外层判空
    for (i, &arg) in expr
      .args
      .as_slice()
      .iter()
      .enumerate()
      .take(min(func_args_size, K_MAX_COST_ARGS))
    {
      if self.is_constant(arg) {
        varc[i] = true;
        has_constant = true;
      }
    }

    // 若尾实参只返回单值，其后所有实参位都按 nil 处理（cpp 同注释）
    // 尾元素经 as_slice().last() 取回（size!=0 即 Some），取代裸 data.add(size-1)
    if let Some(&last_arg) = expr.args.as_slice().last()
      && !self.is_expr_mult_ret(last_arg)
    {
      for flag in varc
        .iter_mut()
        .take(min(func_args_size, K_MAX_COST_ARGS))
        .skip(expr_args_size)
      {
        *flag = true;
        has_constant = true;
      }
    }

    // 若存在能非平凡地影响本次调用成本模型的常量实参
    if has_constant {
      call_cost_model = self.cost_model_inlined_call(expr, func);
    }

    let inlined_cost = compute_cost(
      call_cost_model,
      &varc[..min(func_args_size, K_MAX_COST_ARGS)],
    );
    let baseline_cost = compute_cost(fi_cost_model, &[]) + K_BASELINE_COST_BONUS;
    let inline_profit = if inlined_cost == 0 {
      threshold_max_boost
    } else {
      min(
        threshold_max_boost,
        K_COST_PERCENT_SCALE * baseline_cost / inlined_cost,
      )
    };

    let threshold = threshold_base * inline_profit / K_COST_PERCENT_SCALE;

    if inlined_cost > threshold {
      self.bc_mut().add_debug_remark(format_args!(
        "inlining failed: too expensive (cost {}, profit {:.2}x)",
        inlined_cost,
        inline_profit as f64 / K_COST_PERCENT_SCALE as f64
      ));
      return false;
    }

    // 先读 inline_frames.len()（对 self 的共享借用）再交给 bc_mut，避免两借用同表达式重叠
    let inline_depth = self.inline_frames.len() as i32;
    self.bc_mut().add_debug_remark(format_args!(
      "inlining succeeded (cost {}, profit {:.2}x, depth {})",
      inlined_cost,
      inline_profit as f64 / K_COST_PERCENT_SCALE as f64,
      inline_depth
    ));

    self.compile_inlined_call(expr, func, target, target_count);
    true
  }

  /// 调用前提为 `inline_frames` 栈非空（本函数 `last().expect` 取帧），即必须
  /// 处于 `compile_inlined_call` 建立的帧作用域内，否则帧栈失配产出错误返回值
  /// 寄存器（正确性约定，非内存安全前提）。
  pub(crate) fn compile_inline_return(&mut self, stat_ref: &AstStatReturn, _fallthrough: bool) {
    self.set_debug_line_ast_node(&stat_ref.base.base);

    let frame = self
      .inline_frames
      .last()
      .expect("inline_frames must not be empty")
      .clone();

    self.compile_expr_list_temp(&stat_ref.list, frame.target, frame.target_count, false);

    self.close_locals(frame.local_offset);

    let jump_label = self.bc().emit_label();
    self.bc_mut().emit_ad(LuauOpcode::LOP_JUMP, 0, 0);

    self
      .inline_frames
      .last_mut()
      .expect("inline_frames must not be empty")
      .return_jumps
      .push(jump_label);
  }

  /// 试算内联代价：临时把常量实参值登记进 locstants、折叠 body 后跑 modelCost，
  /// 随后完整回滚，保证不影响真实编译状态。
  ///
  /// `expr`/`func` 须为内联判定路径成对传入的 arena 存活节点，且 `func` 确为
  /// `expr` 的被调 `AstExprFunction`——args 切片、body 视图都从这一对应关系派生；
  /// 不匹配时代价估算失真会误导 `try_compile_inlined_call` 的内联决策（决策正确性
  /// 前提，非内存安全前提）。
  pub(crate) fn cost_model_inlined_call(
    &mut self,
    expr_ref: &AstExprCall,
    func_ref: &AstExprFunction,
  ) -> u64 {
    for (i, var) in func_ref.args.iter().enumerate() {
      // cpp `i < args.size ? args.data[i] : nullptr` 下标游标+补位哨兵折叠为
      // 切片 get()/Option：`None` 即“该实参缺失”；filter 兜底保持与 cpp
      // 对数组内 null 元素的同构语义（parser 不变式下通常不触发）。
      let arg = expr_ref
        .args
        .as_slice()
        .get(i)
        .copied()
        .filter(|p| !p.is_null());

      if i + 1 == expr_ref.args.len()
        && func_ref.args.len() > expr_ref.args.len()
        && arg.is_some_and(|a| self.is_expr_mult_ret(a))
      {
        break;
      }

      if self
        .variables
        .find(&Node::from(var))
        .is_some_and(|vv| vv.written)
      {
        continue;
      }

      match arg {
        None => *self.locstants.get_or_insert(Node::from(var)) = Constant::Nil,
        Some(a) => {
          if let Some(cv) = self.constants.find(&a.into())
            && !cv.is_unknown()
          {
            *self.locstants.get_or_insert(Node::from(var)) = *cv;
          }
        }
      }
    }

    self.expr_changes.clear();
    self.local_changes.clear();

    // Safety: func_ref.body 为 parser 在 arena 接线的函数体根（非空、比编译存活）；
    // 折叠/代价访问需要 body 子树的独占视图（cpp 侧为 const_cast），func_ref 的共享借用
    // 仅覆盖 func 节点头部字段（args/body 读取），两借用指向互不相交的节点内存。
    let body = unsafe { &mut *func_ref.body.as_ptr() };

    self.fold_constants(&mut body.base.base, true);

    // 形参指针切片升格为地址句柄切片（仅作文本变量折扣的 map 键，见 `model_cost`）
    let cost = model_cost(
      &mut body.base.base,
      &func_ref
        .args
        .iter_nodes()
        .map(|n| Node::from(*n))
        .collect::<Vec<_>>(),
      &self.builtins,
      &self.constants,
    );

    for arg in func_ref.args.iter() {
      if let Some(var) = self.locstants.find_mut(&Node::from(arg)) {
        *var = Constant::Unknown;
      }
    }

    undo_changes_expr(&mut self.constants, &self.expr_changes);
    undo_changes_local(&mut self.locstants, &self.local_changes);

    cost
  }

  /// cpp `shouldShareFunction`：闭包上值全部未被改写、且来自单一父函数时可共享。
  pub(crate) fn should_share_closure(&mut self, func: impl Into<Node<AstExprFunction>>) -> bool {
    let func = func.into();

    // 上值句柄表规模有硬上限（K_MAX_UPVALUE_COUNT=200）：拷出地址句柄后
    // self 借用即归还，循环内的递归 `&mut self` 与表读查询互不冲突
    // （对应 cpp 直接 range-for `f->upvals`，无需裸指针切片逃逸）。
    let upvals: Vec<Node<AstLocal>> = match self.functions.find(&func) {
      Some(f) => f.upvals.clone(),
      None => return false,
    };

    for uv in upvals {
      let ul = match self.variables.find(&uv) {
        Some(ul) => *ul,
        None => return false,
      };

      if ul.written {
        return false;
      }

      let nested = {
        let uv_ref = uv.borrow();
        uv_ref.function_depth != 0 || uv_ref.loop_depth != 0
      };

      if nested {
        // cpp `ul->init ? ul->init->as<AstExprFunction>() : nullptr` → Option 链，
        // 无初值或类型不符均视为失败。
        let uf = ul.init.and_then(|init| {
          ast_node_try_as::<AstExprFunction>(&init.borrow().base).map(Node::from_ref)
        });

        let Some(uf) = uf else {
          return false;
        };

        if uf != func && !self.should_share_closure(uf) {
          return false;
        }
      }
    }

    true
  }

  /// 沿局部变量初值/包装表达式（group、类型断言、instantiation、常量表索引）
  /// 下钻，解析出节点背后的函数表达式；未命中返回 `None`（取代 cpp 的 null 哨兵）。
  ///
  /// 只读分支走判型下转门面（判空 + 下转收口一处）；控制流、查表与递归都是安全代码。
  pub(crate) fn get_function_expr(
    &self,
    node: impl Into<Node<AstExpr>>,
  ) -> Option<Node<AstExprFunction>> {
    let node = node.into();
    let node_ptr = node.as_ptr();
    // 门面判型+下转：命中即动态类型 AstExprLocal；expr_local.local 是 parser
    // 登记、编译期只读存活的 AstLocal。
    if let Some(expr_local) = ast_slot_try_as::<AstExprLocal, _>(node_ptr) {
      let lv = self.variables.find(&expr_local.local.into()).copied();
      if lv.is_none_or(|lv| lv.written) {
        return None;
      }

      // cpp `!lv->init` 判空 → Option（无初值即非函数表达式，`?` 直接上抛 None）
      let init = lv.and_then(|lv| lv.init)?;
      return self.get_function_expr(init);
    }

    // 门面判型+下转：命中即 AstExprIndexName；try_index_constant_table 只返回
    // Some(同一存活 AST 中的表达式指针) 或 None，递归沿树下行深度有限。
    if let Some(expr_index) = ast_slot_try_as::<AstExprIndexName, _>(node_ptr)
      && fflag::LuauCompileInlineTableFunctions.get()
    {
      let value = self.try_index_constant_table(expr_index);
      if let Some(value) = value {
        return self.get_function_expr(value);
      }

      return None;
    }

    // 门面判型+下转：group.expr 为 parser 保证非空存活的子节点，位于 arena、编译期只读存活。
    if let Some(expr_group) = ast_slot_try_as::<AstExprGroup, _>(node_ptr) {
      return self.get_function_expr(expr_group.expr);
    }

    // 门面判型+下转：assertion.expr 由 parser 接线为非空子节点。
    if let Some(expr_assertion) = ast_slot_try_as::<AstExprTypeAssertion, _>(node_ptr) {
      return self.get_function_expr(expr_assertion.expr);
    }

    // 门面判型+下转：instantiate.expr 由 parser 接线为非空子节点。
    if let Some(expr_instantiate) = ast_slot_try_as::<AstExprInstantiate, _>(node_ptr)
      && fflag::LuauCompileInlineTableFunctions.get()
    {
      return self.get_function_expr(expr_instantiate.expr);
    }

    // Safety: 命中即动态类型为 AstExprFunction（repr(C) 首字段与基类重合）；`Node::from_mut`
    // 把该独占借用的地址升格为句柄交回 `Option<Node<_>>` 查询签名，借用随表达式求值终止。
    unsafe { ast_node_try_as_ptr_mut::<AstExprFunction>(node_ptr) }.map(Node::from_mut)
  }

  /// `local` 以引用证明存活（parser arena 节点或 Compiler 内嵌 `export_table_local`
  /// 字段）：upvals 表按节点句柄查重，报错路径读 location/name 走安全借用。
  pub(crate) fn get_upval(&mut self, local: &AstLocal) -> u8 {
    let local_node = Node::from_ref(local);
    if let Some(uid) = self.upvals.iter().position(|&upval| upval == local_node) {
      return uid as u8;
    }

    if self.upvals.len() >= K_MAX_UPVALUE_COUNT {
      CompileError::raise(
        &local.location,
        format_args!(
          "Out of upvalue registers when trying to allocate {}: exceeded limit {}",
          local.name, K_MAX_UPVALUE_COUNT
        ),
      );
    }

    if self.variables.find(&local_node).is_some_and(|v| v.written) {
      self.locals.get_or_insert(local_node).captured = true;
    }

    self.upvals.push(local_node);
    (self.upvals.len() - 1) as u8
  }

  /// `body` 允许 None（cpp null 早退的 Option 化）；Some 引用证明块节点存活，
  /// 其语句数组元素由 parser 保证非空存活。须在
  /// `DebugLuauUserDefinedClasses` 开启时调用（入口断言）。
  pub(crate) fn preallocate_hoisted_classes(&mut self, body: Option<&AstStatBlock>) {
    LUAU_ASSERT!(fflag::DebugLuauUserDefinedClasses.get());

    let Some(body) = body else {
      return;
    };

    for &stat in body.body.iter_nodes() {
      // Safety: 语句槽为 parser 构造、编译期存活的非空节点指针；
      // ast_node_try_as_ptr 先经 RTTI class index 校验，命中者确为 AstStatClass
      // （repr(C) 首字段布局一致），decl.name 是其登记的存活 AstLocal。
      unsafe {
        if let Some(decl) = ast_node_try_as_ptr::<AstStatClass>(stat) {
          let reg = self.alloc_reg(&decl.base.base, 1);
          self.push_local(&*decl.name, reg, K_DEFAULT_ALLOC_PC);
          self.bc_mut().emit_abc(LuauOpcode::LOP_LOADNIL, reg, 0, 0);
        }
      }
    }
  }

  /// `name`/`func` 已句柄化为 Node（parser 保证非空存活）；`as_ptr().cast()` 依赖
  /// `AstExprFunction` 首字段为 `AstExpr` 的 repr(C) 布局。
  pub(crate) fn compile_stat_function(&mut self, stat_ref: &AstStatFunction) {
    if let reg = self.get_expr_local_reg(stat_ref.name)
      && reg >= 0
    {
      // Safety: func 句柄出自 arena 存活契约，as_ptr 桥回裸指针后重建 `&mut`
      // 与 cpp 直传 `stat->func` 同语义（编译链对该节点写穿寄存器分配）。
      self.compile_expr(
        unsafe { &mut *stat_ref.func.as_ptr().cast::<AstExpr>() },
        reg as u8,
        false,
      );
      return;
    }

    let mut rs = self.reg_scope();
    let reg = self.alloc_reg(&stat_ref.base.base, 1);
    // Safety: 同上，func 指针存活且类型正确。
    self.compile_expr(
      unsafe { &mut *stat_ref.func.as_ptr().cast::<AstExpr>() },
      reg,
      true,
    );
    // name 已句柄化为 Node<AstExpr>（非空+存活由句柄契约承载），`as_ptr` 桥交
    // 仍以指针形态消费的 compile_l_value/compile_assign。
    let var = self.compile_l_value(stat_ref.name.as_ptr(), &mut rs);
    self.compile_assign(&var, reg, Some(stat_ref.name.as_ptr()));
  }

  /// `func_ref` 由调用方 `&mut` 交接证明存活且独占（`body` 非空由 parser 保证）；
  /// visitor 只写自身 upvals 收集表、不写块节点。
  pub(crate) fn gather_const_upvals(&mut self, func_ref: &mut AstExprFunction) {
    let upvals = {
      let mut visitor = ConstUpvalueVisitor::new(self);
      // body 已句柄化：&mut func_ref 即独占证明，DerefMut 交出块借用；
      // visitor 只写自身 upvals 收集表、不回写块字段。
      ast_stat_block_visit(func_ref.body.get_mut(), &mut visitor);
      // 先取走收集结果、结束对 self 的只读借用，再进入 get_upval 的可变阶段。
      take(&mut visitor.upvals)
    };

    for local in upvals {
      // local 为上一步遍历从 AST 收集的存活节点句柄（契约见 `Node::borrow`）。
      self.get_upval(local.borrow());
    }
  }
}

/// 成本模型位向量最多覆盖的参数数（C++ `bool varc[8]`）
const K_MAX_COST_ARGS: usize = 8;
/// 内联基线成本加成（C++ `computeCost(...) + 3`）
const K_BASELINE_COST_BONUS: i32 = 3;
/// 内联允许的最大寄存器压力（C++ `regTop > 128`）
const K_MAX_INLINE_REG_TOP: u32 = 128;
/// 内联允许的最大函数栈帧大小（C++ `stackSize > 32`）
const K_MAX_INLINE_STACK_SIZE: u32 = 32;
/// 利润百分比换算基数（C++ `profit = maxBoost * 100 / cost`）
const K_MAX_UPVALUE_COUNT: usize = 200;
