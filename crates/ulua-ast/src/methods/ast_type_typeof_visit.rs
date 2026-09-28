use crate::{
  records::{ast_type_typeof::AstTypeTypeof, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit},
};

impl_visitable!(AstTypeTypeof, TypeTypeof, |this, visitor| {
  // Safety: expr 是 typeof 解析传入的 arena 存活表达式节点；ast_expr_visit 契约与独占遍历前提成立。
  unsafe {
    ast_expr_visit(this.expr, visitor);
  }
});
