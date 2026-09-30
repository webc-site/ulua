use crate::{
  records::{ast_stat_type_function::AstStatTypeFunction, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit},
};

impl_visitable!(AstStatTypeFunction, StatTypeFunction, |this, visitor| {
  // Safety: body 是 parse_function_body 产出的 arena 存活 AstExprFunction 指针，cast 上转仅基址视图变换（repr(C)）；self 与节点同 arena。
  unsafe {
    ast_expr_visit(this.body.cast(), visitor);
  }
});
