use ulua_ast::{enums::ast_expr_ref::AstExprRef, records::ast_expr::AstExpr};

pub(crate) fn unwrap_group(mut expr: *mut AstExpr) -> *mut AstExpr {
  // Safety: expr 指向 parser arena 存活的 AstExpr 节点或为 null；
  // as_ref 先判空，as_expr_ref 基于 repr(C) 基类 class_index 安全模式匹配具体枚举，全程只读。
  while let Some(AstExprRef::Group(group)) = (unsafe { expr.as_ref() }).map(AstExpr::as_expr_ref) {
    expr = group.expr.as_ptr();
  }

  expr
}
