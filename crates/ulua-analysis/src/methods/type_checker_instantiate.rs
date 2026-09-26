use core::ptr::null_mut;

use ulua_ast::records::location::Location;

use crate::{
  functions::{arc_as_mut::arc_as_mut, follow_type, get_type},
  records::{
    arena_handle::Handle, function_type::FunctionType, txn_log::TxnLog, type_checker::TypeChecker,
    unification_too_complex::UnificationTooComplex,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl TypeChecker {
  pub fn instantiate(
    &mut self,
    scope: &ScopePtr,
    ty: TypeId,
    location: Location,
    log: *const TxnLog,
  ) -> TypeId {
    let ty = follow_type::follow(ty);

    if let Some(ftv) = get_type::get::<FunctionType>(ty)
      && ftv.has_no_free_or_generic_types
    {
      return ty;
    }

    // reusableInstantiation.resetState(log, &currentModule->internalTypes, builtinTypes, scope->level, /*scope*/ nullptr);
    unsafe {
      let arena = Handle::from_mut(
        &mut (*(arc_as_mut(self.expect_current_module()))).internal_types,
      );
      self.reusable_instantiation.reset_state(
        log,
        arena,
        self.builtin_types,
        scope.level,
        null_mut(),
      );
    }

    if let Some(child_limit) = self.instantiation_child_limit {
      self.reusable_instantiation.base.base.child_limit = child_limit;
    }

    let instantiated = self.reusable_instantiation.substitute_type_id(ty);

    if let Some(instantiated) = instantiated {
      instantiated
    } else {
      self
        .report_error_location_type_error_data(&location, UnificationTooComplex::default().into());
      self.error_recovery_type_scope_ptr(scope)
    }
  }
}
