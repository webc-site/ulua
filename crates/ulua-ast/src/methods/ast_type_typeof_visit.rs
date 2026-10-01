use crate::{
  records::{ast_type_typeof::AstTypeTypeof, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstTypeTypeof, TypeTypeof, |this, visitor| {
  // expr 槽已句柄化：get_mut 沿 &mut self 交出独占子节点引用，引用门面递归，全链路无裸指针。
  ast_expr_visit_ref(this.expr.get_mut(), visitor);
});
