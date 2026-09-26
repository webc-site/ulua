use crate::records::{
  arena_handle::Handle, builtin_types::BuiltinTypes,
  internal_error_reporter::InternalErrorReporter, subtyping_unifier::SubtypingUnifier,
  type_arena::TypeArena,
};

impl SubtypingUnifier {
  pub fn new(
    arena: Handle<TypeArena>,
    builtin_types: Handle<BuiltinTypes>,
    reporter: *mut InternalErrorReporter,
  ) -> Self {
    Self {
      arena,
      builtin_types,
      reporter: Handle::from_ptr(reporter),
    }
  }
}
