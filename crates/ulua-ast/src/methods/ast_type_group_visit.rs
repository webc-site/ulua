use crate::{
  records::{ast_type_group::AstTypeGroup, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit},
};

impl_visitable!(AstTypeGroup, TypeGroup, |this, visitor| {
  // Safety: type_ 是 parse_type 链传入的 arena 存活节点（分组必有内层类型）；ast_type_visit 契约满足，独占遍历。
  unsafe {
    ast_type_visit(this.type_, visitor);
  }
});
