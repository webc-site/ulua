//! `Compiler` 寄存器与局部作用域：`alloc_reg`/出入栈 locals、RAII reg_scope 守卫、
//! 调试行号与类型提示（对照 cpp `Compiler.cpp` 的 pushLocal/allocReg/setDebugLine 段）。
use core::{cmp::max, ptr::from_mut};

use ulua_ast::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_local::AstLocal,
    ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock,
    ast_stat_local::AstStatLocal, ast_stat_type_alias::AstStatTypeAlias, location::Location,
  },
  visit::ast_expr_visit,
};
use ulua_common::{
  enums::{luau_bytecode_type::LuauBytecodeType, luau_opcode::LuauOpcode},
  fflag::DebugLuauUserDefinedClasses,
  macros::luau_assert::LUAU_ASSERT,
};

use crate::{
  enums::kind::Kind,
  functions::{
    ast_slot_ref::{ast_slot_ref, ast_slot_try_as},
    sref_compiler::sref_ast_name,
  },
  records::{
    assignment::Assignment,
    compile_error::CompileError,
    compiler::{Compiler, K_DEFAULT_ALLOC_PC, K_MAX_REGISTER_COUNT},
    node::Node,
    reg_scope::RegScope,
    visitor::Visitor,
  },
};
impl Compiler {
  /// cpp Compiler.cpp `allocReg`：分配连续寄存器；`node` 仅用于越界时定位报错。
  pub fn alloc_reg(&mut self, node: &AstNode, count: u32) -> u8 {
    let top = self.reg_top;

    if top + count > K_MAX_REGISTER_COUNT {
      // 对应 C++ `CompileError::raise(...)`：throw 类型化 CompileError
      // （panic_any），而非裸 String panic，让 `compile()` 的 catch 能取回它。
      CompileError::raise(
        &node.location,
        format_args!(
          "Out of registers when trying to allocate {} registers: exceeded limit {}",
          count, K_MAX_REGISTER_COUNT
        ),
      );
    }

    self.reg_top += count;
    self.stack_size = max(self.stack_size, self.reg_top);

    top as u8
  }

  pub(crate) fn are_locals_captured(&mut self, start: usize) -> bool {
    LUAU_ASSERT!(start <= self.local_stack.len());

    self.local_stack[start..].iter().any(|&local| {
      let l = self.locals.find(&local);
      LUAU_ASSERT!(l.is_some());
      l.is_some_and(|l| l.captured)
    })
  }

  /// 对应 cpp `areLocalsRedundant`（cpp/Compiler/src/Compiler.cpp:4098）：判定
  /// `local` 语句是否纯冗余（值个数与变量名一致、无导出、全部 constant），供
  /// `compile_stat_local` 在优化路径整体跳过。
  ///
  /// cpp 形参是裸 `AstStatLocal*`（并有判空早退的历史版本），此处以 `&AstStatLocal`
  /// 由类型证明非空——唯一调用方 `compile_stat_local` 本就持有引用。
  pub(crate) fn are_locals_redundant(&mut self, stat: &AstStatLocal) -> bool {
    // 多余表达式可能有副作用
    if stat.values.len() > stat.vars.len() {
      return false;
    }

    // 槽位判空与只读解析收口在 `ast_slot_ref`；`variables` 仍以槽位地址为键查询，
    // 与 map 既有指针一致。
    stat.vars.iter().all(|&slot| {
      let Some(local) = ast_slot_ref(slot) else {
        return false;
      };

      // 导出的 local 必须写入导出表
      if local.is_exported {
        return false;
      }

      self
        .variables
        .find(&slot.into())
        .is_some_and(|v| v.constant)
    })
  }

  pub(crate) fn at_top_level(&self) -> bool {
    // cpp `currentFunction && currentFunction->functionDepth == 0`：Option 化后
    // `is_some_and` 一步承担判空与读取；句柄解引用收口在 `Node::borrow` 契约内。
    self
      .current_function
      .is_some_and(|f| f.borrow().function_depth == 0)
      && self.block_depth == 0
      && self.loops.is_empty()
  }

  pub(crate) fn close_locals(&mut self, start: usize) {
    LUAU_ASSERT!(start <= self.local_stack.len());
    let mut captured = false;
    let mut capture_reg = 255u8;
    for &local_ptr in &self.local_stack[start..] {
      if let Some(l) = self.locals.find(&local_ptr)
        && l.captured
      {
        captured = true;
        capture_reg = capture_reg.min(l.reg);
      }
    }
    if captured {
      self
        .bc_mut()
        .emit_abc(LuauOpcode::LOP_CLOSEUPVALS, capture_reg, 0, 0);
    }
  }

  pub(crate) fn pop_locals(&mut self, start: usize) {
    LUAU_ASSERT!(start <= self.local_stack.len());

    for i in start..self.local_stack.len() {
      let local = self.local_stack[i];
      let l = self.locals.find_mut(&local);
      LUAU_ASSERT!(l.is_some());
      let l = l.unwrap();
      LUAU_ASSERT!(l.allocated);

      l.allocated = false;
      // 提前取走 Copy 字段，结束对 self.locals 的可变借用，让下方 bc/bc_mut
      // 访问器能以 &mut self 形态调用（原裸指针解引用不借 self，故无冲突）。
      let reg = l.reg;
      let debugpc_of_local = l.debugpc;
      let allocpc = l.allocpc;

      if self.options.debug_level >= 2 {
        // local 取自 self.local_stack，其元素只由 push_local 压入，均为
        // parser arena 中存活的 AstLocal 或 Compiler 自身 export_table_local 字段
        // 地址，两者在整个编译期可读（存活契约见 `Node::borrow`）；此处仅复制
        // name 值（Copy 读）交给 sref_ast_name 写调试信息，不修改 AST。
        let name = local.borrow().name;
        let debugpc = self.bc().get_debug_pc();
        self
          .bc_mut()
          .push_debug_local(sref_ast_name(name), reg, debugpc_of_local, debugpc);
      }

      if self.options.type_info_level >= 1 && i >= self.arg_count {
        let debugpc = self.bc().get_debug_pc();
        let ty = self
          .local_types
          .find(&local)
          .copied()
          .unwrap_or(LuauBytecodeType::LBC_TYPE_ANY);

        self
          .bc_mut()
          .push_local_type_info(ty, reg, allocpc, debugpc);
      }
    }

    // cpp `localStack.resize(start)`：截断回 start 长（收缩路径从不使用填充值，
    // 原 null_mut() 填充实为死参），truncate 即其 Rust 对应。
    self.local_stack.truncate(start);
  }

  /// `local` 以引用证明存活（parser arena 节点或 Compiler 内嵌 `export_table_local`
  /// 字段）；`reg` 须为 `alloc_reg` 刚分配、`allocpc` 为该分配点 pc——二者成对
  /// 写入 local_stack，出块时据此还原水位。地址键由借用升格为节点句柄。
  pub fn push_local(&mut self, local: &AstLocal, reg: u8, allocpc: u32) {
    if self.local_stack.len() >= K_MAX_LOCAL_COUNT {
      CompileError::raise(
        &local.location,
        format_args!(
          "Out of local registers when trying to allocate {}: exceeded limit {}",
          local.name, K_MAX_LOCAL_COUNT
        ),
      );
    }

    let local_node = Node::from_ref(local);
    self.local_stack.push(local_node);

    let debugpc = self.bc().get_debug_pc();
    let l = self.locals.get_or_insert(local_node);
    LUAU_ASSERT!(!l.allocated);

    l.reg = reg;
    l.allocated = true;
    l.debugpc = debugpc;
    l.allocpc = if allocpc == K_DEFAULT_ALLOC_PC {
      l.debugpc
    } else {
      allocpc
    };
  }

  pub(crate) fn get_local_reg(&self, local: impl Into<Node<AstLocal>>) -> i32 {
    match self.locals.find(&local.into()) {
      Some(l) if l.allocated => i32::from(l.reg),
      _ => -1,
    }
  }

  pub(crate) fn get_expr_local_reg(&mut self, node: impl Into<Node<AstExpr>>) -> i32 {
    let node = node.into();
    {
      if let Some(expr) = self.get_expr_local(node) {
        // get_expr_local 经 RTTI 类型校验返回 Some(arena 中确为 AstExprLocal 的
        // 节点句柄)，命中后 .local 读取合法；该 local 是 parser 登记、编译期存活
        // 的 AstLocal，locals 仅以句柄为键查询，不改写不释放。
        let local = expr.borrow().local;
        match self.locals.find(&local.into()) {
          Some(l) if l.allocated => l.reg as i32,
          _ => -1,
        }
      } else if DebugLuauUserDefinedClasses.get()
        // 门面判型+下转：null/不符返回 None，命中即动态类型
        // AstExprGlobal，g.name 仅在 Some 分支读取；class_locals 只以名字为键查询。
        && let Some(g) = ast_slot_try_as::<AstExprGlobal, _>(node.as_ptr())
        && let Some(&local) = self.class_locals.find(&g.name)
      {
        self.get_local_reg(local)
      } else {
        -1
      }
    }
  }

  /// cpp Compiler.cpp `needsCoverage`：仅按节点动态类型判定，无需可变借用。
  pub(crate) fn needs_coverage(&self, node: &AstNode) -> bool {
    !(node.is::<AstStatBlock>() || node.is::<AstStatTypeAlias>())
  }

  pub(crate) fn set_debug_line_ast_node(&mut self, node: &AstNode) {
    if self.options.debug_level >= 1 {
      let line = node.location.begin.line + 1;
      self.bc_mut().set_debug_line(line as i32);
    }
  }

  pub(crate) fn set_debug_line_location(&mut self, location: &Location) {
    if self.options.debug_level >= 1 {
      self
        .bc_mut()
        .set_debug_line((location.begin.line + 1) as i32);
    }
  }

  pub(crate) fn set_debug_line_end(&mut self, node: &AstNode) {
    if self.options.debug_level >= 1 {
      let line = node.location.end.line + 1;
      self.bc_mut().set_debug_line(line as i32);
    }
  }

  /// 对应 cpp `Compiler::resolveAssignConflicts`。原裸指针契约 fn 已 safe 化：
  /// `stat` 由 compile_stat_assign 现场传入（arena 存活语句指针，基类字段经
  /// 槽位门面读取作 alloc_reg 定位）；`vars`/`values` 为该赋值语句的有效借出
  /// 切片，元素为 parser 登记的表达式指针——`ast_expr_visit` 期间解引用由
  /// ulua-ast 分发门面收口；位图下标依赖 reg/index < 256（u8 寄存器号）天然不越界。
  pub(crate) fn resolve_assign_conflicts(
    &mut self,
    stat: &AstStat,
    vars: &mut [Assignment],
    values: &AstArray<*mut AstExpr>,
  ) {
    // 位图下标依赖 reg/index 均来自 u8 寄存器号（<256），[u64;4] 不会越界；
    // 扫描三 pass 均为只读遍历 + visitor 位图累加。
    let conflict = {
      let mut visitor = Visitor::new(self);

      for (i, var) in vars.iter().enumerate() {
        let li = &var.lvalue;
        if li.kind == Kind::Local {
          if let Some(&expr) = values.as_slice().get(i) {
            // Safety: values 是调用方借出的有效切片，元素为 parser 登记的存活表达式
            // 指针；visitor 只读借用 Compiler（调用方独占），满足 ast_expr_visit 契约。
            unsafe { ast_expr_visit(expr, &mut visitor) };
          }
          let reg = li.reg as usize;
          visitor.assigned[reg / 64] |= 1 << (reg % 64);
        }
      }

      // 原 pass2（非 Local 左值对应的 value）与 pass3（多余尾部 value）合并为
      // 单遍：pass2 为升序下标、pass3 从 vars.len() 起续排，合并后 visit 顺序
      // 与原两遍完全一致（Local 槽位延后到 pass1 已处理）。
      for (i, &expr) in values.iter().enumerate() {
        let local_owned = vars
          .get(i)
          .is_some_and(|var| var.lvalue.kind == Kind::Local);
        if !local_owned {
          // Safety: 同上，expr 为 values 切片内 parser 登记的存活表达式指针，
          // visitor 全程只读、无别名冲突。
          unsafe { ast_expr_visit(expr, &mut visitor) };
        }
      }

      for var in vars.iter() {
        let li = &var.lvalue;
        if matches!(
          li.kind,
          Kind::IndexName | Kind::IndexNumber | Kind::IndexExpr
        ) {
          let reg = li.reg as usize;
          if (visitor.assigned[reg / 64] & (1 << (reg % 64))) != 0 {
            visitor.conflict[reg / 64] |= 1 << (reg % 64);
          }
        }
        if li.kind == Kind::IndexExpr {
          let idx = li.index as usize;
          if (visitor.assigned[idx / 64] & (1 << (idx % 64))) != 0 {
            visitor.conflict[idx / 64] |= 1 << (idx % 64);
          }
        }
      }

      // visitor 只读借用 Compiler；位图取出后即结束借用，令下方 alloc_reg 的
      // 可变借用合法（原裸指针形态靠契约掩盖的别名在此显式化）。
      visitor.conflict
    };

    for var in vars.iter_mut() {
      let li = &var.lvalue;
      if li.kind == Kind::Local {
        let reg = li.reg as usize;
        if (conflict[reg / 64] & (1 << (reg % 64))) != 0 {
          var.conflict_reg = self.alloc_reg(&stat.base, 1);
        }
      }
    }
  }

  pub(crate) fn hint_temporary_reg_type(
    &mut self,
    expr: *mut AstExpr,
    reg: i32,
    expected_type: LuauBytecodeType,
    inst_length: i32,
  ) {
    // LuauBytecodeType 为 Copy：copied() 取走值后借用结束，bc/bc_mut 可安全成链。
    if let Some(ty) = self.expr_types.find(&expr.into()).copied()
      && ty != expected_type
    {
      let debug_pc = self.bc().get_debug_pc();
      self
        .bc_mut()
        .push_local_type_info(ty, reg as u8, debug_pc - inst_length as u32, debug_pc);
    }
  }

  /// NUMBER/宽 1 提示便捷入口：`hint_temporary_expr_reg_type(.., LBC_TYPE_NUMBER, 1)`
  /// 重复点位的单行收口，寄存器编号统一 u8 形态。
  pub(crate) fn hint_number_reg(&mut self, expr: *mut AstExpr, reg: u8) {
    self.hint_temporary_expr_reg_type(expr, reg as i32, LuauBytecodeType::LBC_TYPE_NUMBER, 1);
  }

  /// TABLE 提示便捷入口（表对象操作数的指令宽 `inst_length` 为 1/2）：
  /// 同 [`Self::hint_number_reg`] 的收口形态。
  pub(crate) fn hint_table_reg(&mut self, expr: *mut AstExpr, reg: u8, inst_length: i32) {
    self.hint_temporary_expr_reg_type(
      expr,
      reg as i32,
      LuauBytecodeType::LBC_TYPE_TABLE,
      inst_length,
    );
  }

  pub(crate) fn hint_temporary_expr_reg_type(
    &mut self,
    expr: *mut AstExpr,
    reg: i32,
    expected_type: LuauBytecodeType,
    inst_length: i32,
  ) {
    // 若该操作数参数占用了临时寄存器（即不是局部变量），则尝试为其提示类型
    if self.get_expr_local(expr).is_none() {
      self.hint_temporary_reg_type(expr, reg, expected_type, inst_length);
    }
  }

  /// 对应 cpp `RegScope rs(this)`：登记当前 reg_top 水位，守卫析构时回卷。
  /// 返回的 RegScope 持 `self` 裸地址（契约见 `RegScope` 文档：生命周期
  /// 严格嵌套于本可变借用），调用方不得让其逃逸出本借用作用域。
  pub(crate) fn reg_scope(&mut self) -> RegScope {
    RegScope {
      self_: from_mut(self),
      old_top: self.reg_top,
    }
  }

  /// 对应 cpp `RegScope rs(this, top)`：压低 reg_top 至 target 并在析构回卷。
  /// 裸地址契约同上条 `reg_scope`。
  pub(crate) fn reg_scope_top(&mut self, top: u32) -> RegScope {
    assert!(top <= self.reg_top);
    let old_top = self.reg_top;
    self.reg_top = top;
    RegScope {
      self_: from_mut(self),
      old_top,
    }
  }
}

const K_MAX_LOCAL_COUNT: usize = 200;
