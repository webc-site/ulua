use core::ptr::{NonNull, null_mut};

use crate::records::{
  builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter, scope::Scope,
  type_arena::TypeArena, unifier_2::Unifier2,
};
impl Unifier2 {
  pub fn unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter(
    arena: NonNull<TypeArena>,
    builtin_types: NonNull<BuiltinTypes>,
    scope: NonNull<Scope>,
    ice: NonNull<InternalErrorReporter>,
  ) -> Self {
    Self::unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter_dense_hash_set_void(
            arena,
            builtin_types,
            scope,
            ice,
            null_mut(),
        )
  }
}
