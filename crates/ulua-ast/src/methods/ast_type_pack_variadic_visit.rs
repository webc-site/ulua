use crate::{
  records::{ast_type_pack_variadic::AstTypePackVariadic, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit_ref},
};

impl_visitable!(AstTypePackVariadic, TypePackVariadic, |this, visitor| {
  // variadic_type 槽已句柄化：get_mut 沿 &mut self 交出独占子节点引用，引用门面递归，全链路无裸指针。
  ast_type_visit_ref(this.variadic_type.get_mut(), visitor);
});
