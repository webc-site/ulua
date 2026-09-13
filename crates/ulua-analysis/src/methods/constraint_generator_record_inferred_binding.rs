//! @interface-stub
use core::ptr::null_mut;

use ulua_ast::records::{ast_local::AstLocal, location::Location};
use ulua_common::records::dense_hash_table::DenseDefault;

use crate::{
  records::{
    constraint_generator::{ConstraintGenerator, InferredBinding},
    symbol::Symbol,
    type_ids::TypeIds,
  },
  type_aliases::type_id::TypeId,
};
impl DenseDefault for InferredBinding {
  fn dense_default() -> Self {
    Self {
      scope: null_mut(),
      location: Location::default(),
      types: TypeIds::new(),
    }
  }
}

impl ConstraintGenerator {
  pub fn record_inferred_binding(&mut self, local: *mut AstLocal, ty: TypeId) {
    if let Some(ib) = self.inferred_bindings.find_mut(&Symbol::from_local(local)) {
      ib.types.insert_type_id(ty);
    }
  }
}
