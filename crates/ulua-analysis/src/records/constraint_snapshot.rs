use alloc::{string::String, vec::Vec};

use ulua_ast::records::location::Location;
use ulua_common::records::dense_hash_table::DenseDefault;

use crate::records::constraint_block::ConstraintBlock;
#[derive(Debug, Clone)]
pub struct ConstraintSnapshot {
  pub stringification: String,
  pub location: Location,
  pub blocks: Vec<ConstraintBlock>,
}

// A `DenseHashMap<*const Constraint, ConstraintSnapshot>` value-initializes the
// slot on `operator[]` (C++ `target.unsolvedConstraints[c.get()]`), i.e. a
// default-constructed `ConstraintSnapshot{}`.
impl DenseDefault for ConstraintSnapshot {
  fn dense_default() -> Self {
    Self {
      stringification: String::new(),
      location: Location::default(),
      blocks: Vec::new(),
    }
  }
}
