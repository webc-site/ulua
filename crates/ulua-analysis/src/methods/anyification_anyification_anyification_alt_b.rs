use alloc::sync::Arc;

use crate::{
  records::{
    anyification::Anyification, builtin_types::BuiltinTypes,
    internal_error_reporter::InternalErrorReporter, scope::Scope, type_arena::TypeArena,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId},
};
impl Anyification {
  pub fn anyification_type_arena_scope_ptr_not_null_builtin_types_internal_error_reporter_type_id_type_pack_id(
    arena: *mut TypeArena,
    scope: &ScopePtr,
    builtin_types: *mut BuiltinTypes,
    ice_handler: *mut InternalErrorReporter,
    any_type: TypeId,
    any_type_pack: TypePackId,
  ) -> Self {
    let scope_raw = Arc::as_ptr(scope) as *mut Scope;
    Self::anyification_type_arena_not_null_scope_not_null_builtin_types_internal_error_reporter_type_id_type_pack_id(
            arena,
            scope_raw,
            builtin_types,
            ice_handler,
            any_type,
            any_type_pack,
        )
  }
}
