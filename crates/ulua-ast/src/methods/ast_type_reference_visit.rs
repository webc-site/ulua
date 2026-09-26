use crate::{
  functions::visit_type_or_pack_array::visit_type_or_pack_array,
  records::{ast_type_reference::AstTypeReference, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable},
};

impl_visitable!(AstTypeReference, TypeReference, |this, visitor| {
  visit_type_or_pack_array(visitor, this.parameters);
});
