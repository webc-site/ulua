use crate::{
  functions::visit_type_or_pack_array::visit_type_or_pack_array,
  records::{ast_expr_instantiate::AstExprInstantiate, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit},
};

impl_visitable!(AstExprInstantiate, ExprInstantiate, |this, visitor| {
  // Safety: expr 为 parse 链返回的 arena 存活节点（非 null），与 self 同 arena 且地址不移动；类型实参数组经 visit_type_or_pack_array 另行收口。
  unsafe {
    ast_expr_visit(this.expr, visitor);
  }
  visit_type_or_pack_array(visitor, this.type_arguments);
});
