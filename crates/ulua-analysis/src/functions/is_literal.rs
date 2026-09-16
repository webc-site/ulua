use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_function::AstExprFunction,
    ast_expr_table::AstExprTable, ast_node::AstNode,
  },
  rtti::ast_node_is,
};
pub fn is_literal(expr: *const AstExpr) -> bool {
  if expr.is_null() {
    return false;
  }

  let expr = expr as *mut AstExpr;

  if ast_node_is::<AstExprTable>(unsafe { &*(expr as *mut AstNode) }) {
    return true;
  }
  if ast_node_is::<AstExprFunction>(unsafe { &*(expr as *mut AstNode) }) {
    return true;
  }
  if ast_node_is::<AstExprConstantNumber>(unsafe { &*(expr as *mut AstNode) }) {
    return true;
  }
  if ast_node_is::<AstExprConstantString>(unsafe { &*(expr as *mut AstNode) }) {
    return true;
  }
  if ast_node_is::<AstExprConstantBool>(unsafe { &*(expr as *mut AstNode) }) {
    return true;
  }
  if ast_node_is::<AstExprConstantNil>(unsafe { &*(expr as *mut AstNode) }) {
    return true;
  }

  false
}
