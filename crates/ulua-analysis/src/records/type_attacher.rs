use ulua_ast::records::allocator::Allocator;

use crate::{records::module::Module, type_aliases::synthetic_names::SyntheticNames};

#[derive(Debug, Clone)]
pub struct TypeAttacher {
  pub(crate) module: *mut Module,
  pub(crate) allocator: *mut Allocator,
  pub(crate) synthetic_names: SyntheticNames,
}
