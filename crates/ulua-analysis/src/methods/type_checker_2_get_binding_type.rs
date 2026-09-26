use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal},
  rtti::ast_node_try_as,
};

use crate::{
  records::{symbol::Symbol, type_checker_2::TypeChecker2},
  type_aliases::type_id::TypeId,
};

impl TypeChecker2 {
  // cpp TypeChecker2.cpp:1221
  pub fn get_binding_type(&mut self, expr: &AstExpr) -> Option<TypeId> {
    // 栈已句柄化：`s` 为栈顶 `Handle<Scope>`（`?` 已排除空栈），目标由
    // push_stack 自 `module.ast_scopes` 登记的存活作用域，`get()` 解引用
    // 契约集中在 `arena_handle`；`lookup_symbol` 为 `&self` 只读。
    let s = *self.stack.last()?;

    if let Some(local_expr) = ast_node_try_as::<AstExprLocal>(&expr.base) {
      // local 槽已句柄化恒非空；Symbol::from_local 为既有裸指针 API，经 as_ptr 桥接。
      return s
        .get()
        .lookup_symbol(Symbol::from_local(local_expr.local.as_ptr()));
    }

    if let Some(global_expr) = ast_node_try_as::<AstExprGlobal>(&expr.base) {
      return s.get().lookup_symbol(Symbol::from_global(global_expr.name));
    }

    None
  }
}
