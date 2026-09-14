use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_node::AstNode,
  },
  rtti::ast_node_is,
};

pub fn is_constant_literal(expr: *mut AstExpr) -> bool {
  if expr.is_null() {
    return false;
  }

  let node = expr as *mut AstNode;
  unsafe {
    ast_node_is::<AstExprConstantNil>(&*node)
      || ast_node_is::<AstExprConstantBool>(&*node)
      || ast_node_is::<AstExprConstantNumber>(&*node)
      || ast_node_is::<AstExprConstantString>(&*node)
  }
}
