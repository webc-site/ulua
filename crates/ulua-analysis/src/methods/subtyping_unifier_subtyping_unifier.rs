use crate::records::{
  builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter,
  subtyping_unifier::SubtypingUnifier, type_arena::TypeArena,
};

impl SubtypingUnifier {
  pub fn new(
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    reporter: *mut InternalErrorReporter,
  ) -> Self {
    Self {
      arena,
      builtin_types,
      reporter,
    }
  }
}
