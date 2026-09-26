use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_group::AstExprGroup},
  rtti::ast_node_try_as_ptr,
};
pub(crate) fn unwrap_group(mut expr: *mut AstExpr) -> *mut AstExpr {
  while !expr.is_null() {
    // Safety: 循环守卫已确保 `expr` 非空，它指向 parser arena 中存活的 `#[repr(C)]`
    // AstExpr 节点（首字段 base: AstNode 基址重合）；try_as_ptr 先判空再读
    // class_index 甄别，未命中返回 None 且从不解引用，命中即返回存活 AstExprGroup
    // 的只读借用，取回其 `expr` 子句柄的指针（句柄化后恒非空）为只读操作。全程只读。
    let Some(group) = (unsafe { ast_node_try_as_ptr::<AstExprGroup>(expr) }) else {
      break;
    };
    expr = group.expr.as_ptr();
  }

  expr
}
