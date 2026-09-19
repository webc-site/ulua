use core::ptr::null_mut;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_local::AstLocal, ast_node::AstNode,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::functions::cost_model::model_cost;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn model_cost_ast_node_ast_local_usize(
  root: *mut AstNode,
  vars: *const *mut AstLocal,
  var_count: usize,
) -> u64 {
  let builtins = DenseHashMap::new(null_mut::<AstExprCall>());
  let constants = DenseHashMap::new(null_mut::<AstExpr>());

  unsafe { model_cost(root, vars, var_count, &builtins, &constants) }
}
