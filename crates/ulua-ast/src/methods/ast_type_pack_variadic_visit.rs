use crate::{
  records::{ast_type_pack_variadic::AstTypePackVariadic, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit},
};

impl_visitable!(AstTypePackVariadic, TypePackVariadic, |this, visitor| {
  // Safety: variadic_type 由 parse_type_pack 以 parse_type 结果填充（arena 存活节点）；地址稳定，dispatch 独占借用。
  unsafe {
    ast_type_visit(this.variadic_type, visitor);
  }
});
