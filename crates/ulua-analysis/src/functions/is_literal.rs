use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_constant_bool::AstExprConstantBool,
  ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
  ast_expr_constant_string::AstExprConstantString, ast_expr_function::AstExprFunction,
  ast_expr_table::AstExprTable, node_handle::OptNode,
};
pub fn is_literal(expr: *const AstExpr) -> bool {
  // `expr` 槽位仍是裸指针：经句柄门面 `OptNode::from_ptr` 借出基类引用，判型
  // 走生命周期正确的句柄方法 `is`（只读 repr(C) 首字段 class_index；null 折叠
  // 为 false，原 is_null 早退与 is_ptr 边界形态一并由该折叠退役）。
  let expr_node = OptNode::from_ptr(expr.cast_mut());
  if expr_node.is::<AstExprTable>() {
    return true;
  }
  if expr_node.is::<AstExprFunction>() {
    return true;
  }
  if expr_node.is::<AstExprConstantNumber>() {
    return true;
  }
  if expr_node.is::<AstExprConstantString>() {
    return true;
  }
  if expr_node.is::<AstExprConstantBool>() {
    return true;
  }
  if expr_node.is::<AstExprConstantNil>() {
    return true;
  }

  false
}
