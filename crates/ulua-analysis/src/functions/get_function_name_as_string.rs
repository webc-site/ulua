use alloc::string::{String, ToString};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_group::AstExprGroup,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal,
  },
  rtti::ast_node_try_as,
};
/// cpp `getFunctionNameAsString(AstExpr*)` 的引用形态：下行链 `curr` 全程为
/// 共享引用，下转走生命周期正确的 `ast_node_try_as`；`expr`/`group.expr` 子槽
/// 已句柄化（`Node::get` 借出存活视图），借用半径沿入参 `expr` 的借用传递，
/// 不再锻造 'static。全程只读、单线程。
pub fn get_function_name_as_string(expr: &AstExpr) -> Option<String> {
  let mut curr = expr;
  let mut s = String::new();

  loop {
    if let Some(local) = ast_node_try_as::<AstExprLocal>(curr) {
      let mut name = local.local.name.as_str_or_empty().to_string();
      name.push_str(&s);
      return Some(name);
    }

    if let Some(global) = ast_node_try_as::<AstExprGlobal>(curr) {
      let mut name = global.name.as_str_or_empty().to_string();
      name.push_str(&s);
      return Some(name);
    }

    if let Some(indexname) = ast_node_try_as::<AstExprIndexName>(curr) {
      let next = indexname.expr.get();

      let index_str = indexname.index.as_str_or_empty().to_string();

      let mut new_s = String::new();
      new_s.push('.');
      new_s.push_str(&index_str);
      new_s.push_str(&s);
      s = new_s;

      curr = next;
      continue;
    }

    if let Some(group) = ast_node_try_as::<AstExprGroup>(curr) {
      curr = group.expr.get();
      continue;
    }

    return None;
  }
}
