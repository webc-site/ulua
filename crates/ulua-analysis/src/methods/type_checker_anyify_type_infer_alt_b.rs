use alloc::sync::Arc;

use ulua_ast::records::location::Location;

use crate::{
  records::{
    anyification::Anyification, module::Module, type_arena::TypeArena, type_checker::TypeChecker,
    unification_too_complex::UnificationTooComplex,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};
impl TypeChecker {
  pub fn anyify_scope_ptr_type_pack_id_location(
    &mut self,
    scope: ScopePtr,
    ty: TypePackId,
    location: Location,
  ) -> TypePackId {
    let arena = unsafe {
      &mut (*(Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module)).internal_types
        as *mut TypeArena
    };
    let mut anyification = Anyification::anyification_type_arena_scope_ptr_not_null_builtin_types_internal_error_reporter_type_id_type_pack_id(
            arena,
            &scope,
            self.builtin_types,
            self.ice_handler,
            self.any_type,
            self.any_type_pack,
        );
    let any = anyification.base.substitute_type_pack_id(ty);
    if let Some(any) = any {
      any
    } else {
      self
        .report_error_location_type_error_data(&location, UnificationTooComplex::default().into());
      self.error_recovery_type_pack_type_pack_id(self.any_type_pack)
    }
  }
}
