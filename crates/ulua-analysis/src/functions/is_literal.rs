use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_function::AstExprFunction,
    ast_expr_table::AstExprTable, ast_node::AstNode,
  },
  rtti::ast_node_is_ptr,
};
pub fn is_literal(expr: *const AstExpr) -> bool {
  if expr.is_null() {
    return false;
  }

  // 安全化收口：`ast_node_is` 对 `*const AstNode` 形态自带判空（null → class_index
  // 返回 None → false），RTTI 判别只读 repr(C) 首字段 class_index（偏移 0 基址重合），
  // 不再在调用点手搓 `&*ptr` 解引用；expr 由调用方保证为 null 或指向存活 AstExpr。
  let expr = expr.cast::<AstNode>();

  if unsafe { ast_node_is_ptr::<AstExprTable>(expr) } {
    return true;
  }
  if unsafe { ast_node_is_ptr::<AstExprFunction>(expr) } {
    return true;
  }
  if unsafe { ast_node_is_ptr::<AstExprConstantNumber>(expr) } {
    return true;
  }
  if unsafe { ast_node_is_ptr::<AstExprConstantString>(expr) } {
    return true;
  }
  if unsafe { ast_node_is_ptr::<AstExprConstantBool>(expr) } {
    return true;
  }
  if unsafe { ast_node_is_ptr::<AstExprConstantNil>(expr) } {
    return true;
  }

  false
}
