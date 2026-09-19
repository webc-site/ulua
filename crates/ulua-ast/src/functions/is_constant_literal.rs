use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString,
  },
  rtti::ast_node_is,
};

/// cpp `isConstantLiteral(const AstExpr* expr)`：只读判定，共享引用入参，
/// 空指针分支由引用类型消灭（`&AstExpr` 的类索引读取仍走 rtti 的 `AstNodeRef`）。
pub fn is_constant_literal(expr: &AstExpr) -> bool {
  let node = &expr.base;
  // `ast_node_is` 把 `AstNodeRef::class_index` 的 unsafe 收在自身实现里，
  // 引用入参恒非空，此处已是全 safe 形态
  ast_node_is::<AstExprConstantNil>(node)
    || ast_node_is::<AstExprConstantBool>(node)
    || ast_node_is::<AstExprConstantNumber>(node)
    || ast_node_is::<AstExprConstantString>(node)
}
