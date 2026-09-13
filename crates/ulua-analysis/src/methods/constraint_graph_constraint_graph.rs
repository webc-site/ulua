use core::ptr::NonNull;

use crate::records::{builtin_types::BuiltinTypes, constraint_graph::ConstraintGraph};

impl ConstraintGraph {
  pub fn constraint_graph(&mut self, builtin_types: NonNull<BuiltinTypes>) {
    self.builtin_types = builtin_types;
  }
}
