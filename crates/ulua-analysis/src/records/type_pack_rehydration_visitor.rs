use ulua_ast::records::allocator::Allocator;

use crate::{
  records::type_rehydration_visitor::TypeRehydrationVisitor,
  type_aliases::synthetic_names::SyntheticNames,
};

#[derive(Debug, Clone)]
pub struct TypePackRehydrationVisitor {
  pub(crate) allocator: *mut Allocator,
  pub(crate) synthetic_names: *mut SyntheticNames,
  pub(crate) type_visitor: *mut TypeRehydrationVisitor,
}
