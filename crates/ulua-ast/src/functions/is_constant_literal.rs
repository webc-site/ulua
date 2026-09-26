use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString,
  },
  rtti::ast_node_is,
};

/// cpp `isConstantLiteral(const AstExpr* expr)`：只读判定，共享引用入参，
/// 空指针分支由引用类型消灭（`&AstExpr` 经 rtti 的安全视图 trait `AstNodeView` 升基）。
pub fn is_constant_literal(expr: &AstExpr) -> bool {
  let node = &expr.base;
  // `ast_node_is` 纯 safe：class_index 是 `&AstNode` 的普通字段读取，
  // 引用入参恒非空，此处已是全 safe 形态
  ast_node_is::<AstExprConstantNil>(node)
    || ast_node_is::<AstExprConstantBool>(node)
    || ast_node_is::<AstExprConstantNumber>(node)
    || ast_node_is::<AstExprConstantString>(node)
}
