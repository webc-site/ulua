use alloc::string::String;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString,
    ast_expr_error::AstExprError, ast_expr_global::AstExprGlobal,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_node::AstNode,
  },
  rtti::{AstNodeClass, ast_node_try_as},
};
use ulua_common::LUAU_ASSERT;

use crate::{
  functions::{
    arena_ref::arena_ref, ast_node_downcast::ast_node_downcast as lvalue_downcast,
    contains_subscripted_definition::contains_subscripted_definition,
  },
  records::{data_flow_graph_builder::DataFlowGraphBuilder, symbol::Symbol},
  type_aliases::def_id_def::DefId,
};

impl DataFlowGraphBuilder {
  /// cpp `visitLValue(AstExpr*, DefId)`：按左值形态分派并写回 `astDefs`。
  pub fn visit_lvalue(&mut self, expr: &AstExpr, incoming_def: DefId) {
    // cpp 以 `AstExpr*` 作 astDefs 身份键；从共享引用取同一地址仅作键值。
    let expr_ptr: *const AstExpr = expr as *const AstExpr;
    let node: &AstNode = &expr.base;

    let def = match node.class_index {
      AstExprLocal::CLASS_INDEX => {
        self.visit_lvalue_local(lvalue_downcast::<AstExprLocal>(node), incoming_def)
      }
      AstExprGlobal::CLASS_INDEX => {
        self.visit_lvalue_global(lvalue_downcast::<AstExprGlobal>(node), incoming_def)
      }
      AstExprIndexName::CLASS_INDEX => {
        self.visit_lvalue_index_name(lvalue_downcast::<AstExprIndexName>(node), incoming_def)
      }
      AstExprIndexExpr::CLASS_INDEX => {
        self.visit_lvalue_index_expr(lvalue_downcast::<AstExprIndexExpr>(node), incoming_def)
      }
      AstExprError::CLASS_INDEX => {
        self.visit_lvalue_error(lvalue_downcast::<AstExprError>(node), incoming_def)
      }
      _ => {
        LUAU_ASSERT!(false);
        DefId::NULL
      }
    };

    *self.graph.ast_defs.get_or_insert(expr_ptr) = def;
  }

  /// cpp `visitLValue(AstExprLocal*, DefId)`：非 upvalue 时为新值造 def 并更新
  /// bindings/captures；upvalue 走普通表达式路径（避免别名跟踪越界）。
  pub fn visit_lvalue_local(&mut self, l: &AstExprLocal, incoming_def: DefId) -> DefId {
    let scope = self.current_scope();

    if !l.upvalue {
      let subscripted = contains_subscripted_definition(incoming_def);
      let symbol = Symbol::from_local(l.local.as_ptr());
      let updated =
        self
          .def_arena
          .get_mut()
          .fresh_cell(symbol.clone(), l.base.base.location, subscripted);
      // SAFETY: scope 为 current_scope() 所得栈顶活 DfgScope（PinnedStorage
      // 地址稳定、活至 builder 析构）；l.local 由上方 Symbol 按指针值消费。
      // bindings 写在 &mut self 独占路径上，借用止于本语句（对应 cpp
      // `scope->bindings[l->local] = updated`）。
      unsafe { *(*scope).bindings.get_or_insert(symbol.clone()) = updated };
      self
        .captures
        .get_or_insert(symbol)
        .all_versions
        .push(updated);
      updated
    } else {
      // cpp `visitExpr(static_cast<AstExpr*>(l))`：上转基类走带缓存的分派入口。
      self.visit_expr(&l.base).def
    }
  }

  /// cpp `visitLValue(AstExprGlobal*, DefId)`。
  pub fn visit_lvalue_global(&mut self, g: &AstExprGlobal, incoming_def: DefId) -> DefId {
    let scope = self.current_scope();
    let symbol = Symbol::from_global(g.name);
    let subscripted = contains_subscripted_definition(incoming_def);

    let updated =
      self
        .def_arena
        .get_mut()
        .fresh_cell(symbol.clone(), g.base.base.location, subscripted);
    // SAFETY: 同 visit_lvalue_local——scope 为栈顶活 scope，bindings 写在
    // &mut self 独占路径上顺序发生，无并存别名。
    unsafe { *(*scope).bindings.get_or_insert(symbol.clone()) = updated };
    self
      .captures
      .get_or_insert(symbol)
      .all_versions
      .push(updated);
    updated
  }

  /// cpp `visitLValue(AstExprIndexName*, DefId)`：
  /// `scope->props[parentDef][i->index.value] = updated`。
  pub fn visit_lvalue_index_name(&mut self, i: &AstExprIndexName, incoming_def: DefId) -> DefId {
    // expr 已句柄化恒非空；arena_ref 为既有指针门面，经 as_ptr 桥接（判空 panic 分支类型端不可达）。
    let parent_expr = arena_ref(i.expr.as_ptr(), "AstExprIndexName.expr");
    let parent_def = self.visit_expr(parent_expr).def;
    let scope = self.current_scope();
    let index_str = String::from(i.index.as_str_or_empty());
    let subscripted = contains_subscripted_definition(incoming_def);
    let updated = self.def_arena.get_mut().fresh_cell(
      Symbol::from_global(i.index),
      i.base.base.location,
      subscripted,
    );
    // SAFETY: scope 为 current_scope() 所得栈顶活 scope；parent_def 是
    // visit_expr 产图不变量下的合法 DefId（仅按指针值作 props 键）；props
    // 写在 &mut self 独占路径上，借用止于本语句。
    unsafe {
      (*scope)
        .props
        .get_or_insert(parent_def)
        .insert(index_str, updated)
    };
    updated
  }

  /// cpp `visitLValue(AstExprIndexExpr*, DefId)`：字符串字面量下标按名登记
  /// props，否则视为真下标访问（subscripted = true）。
  pub fn visit_lvalue_index_expr(&mut self, i: &AstExprIndexExpr, incoming_def: DefId) -> DefId {
    // expr/index 已句柄化恒非空；arena_ref 为既有指针门面，经 as_ptr 桥接（判空 panic 分支类型端不可达）。
    let parent_expr = arena_ref(i.expr.as_ptr(), "AstExprIndexExpr.expr");
    let parent_def = self.visit_expr(parent_expr).def;
    let index_expr = arena_ref(i.index.as_ptr(), "AstExprIndexExpr.index");
    self.visit_expr(index_expr);

    let scope = self.current_scope();
    if let Some(string) = ast_node_try_as::<AstExprConstantString>(&index_expr.base) {
      let key = String::from_utf8_lossy(string.value.as_bytes()).into_owned();

      let subscripted = contains_subscripted_definition(incoming_def);
      let updated =
        self
          .def_arena
          .get_mut()
          .fresh_cell(Symbol::default(), i.base.base.location, subscripted);
      // SAFETY: scope 为栈顶活 scope（PinnedStorage 地址稳定）；parent_def 为
      // visit_expr 回吐的合法 DefId；props 写借用止于本语句，无并存别名。
      unsafe {
        (*scope)
          .props
          .get_or_insert(parent_def)
          .insert(key, updated)
      };
      updated
    } else {
      let subscripted = true;
      self
        .def_arena
        .get_mut()
        .fresh_cell(Symbol::default(), i.base.base.location, subscripted)
    }
  }

  /// cpp `visitLValue(AstExprError*, DefId)`：错误恢复左值按普通表达式回退。
  pub fn visit_lvalue_error(&mut self, error: &AstExprError, _incoming_def: DefId) -> DefId {
    // cpp `visitExpr(error).def`：上转基类走带缓存的分派入口。
    self.visit_expr(&error.base).def
  }
}
