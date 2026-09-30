use crate::{
  functions::optional_node::slot_opt,
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_varargs::AstExprVarargs,
    temp_vector::TempVector,
  },
  rtti::ast_node_is,
};

/// cpp `isEnoughValues(TempVector<AstExpr*>& values, size_t expected)`
/// (`cpp/Ast/src/Parser.cpp:1307-1316`): 末元素是调用/变参时视为「足够」，否则比较数量。
/// 只读遍历，故借用取 `&`（cpp 的非 const 引用只是历史签名）。
pub fn is_enough_values(values: &TempVector<'_, *mut AstExpr>, expected: usize) -> bool {
  if let Some(&last) = values.last()
    && let Some(last) = slot_opt(last)
    && (ast_node_is::<AstExprCall>(last) || ast_node_is::<AstExprVarargs>(last))
  {
    return true;
  }
  values.len() == expected
}
