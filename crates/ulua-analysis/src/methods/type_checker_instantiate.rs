use alloc::sync::Arc;
use core::ptr::null_mut;

use ulua_ast::records::location::Location;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    function_type::FunctionType, module::Module, txn_log::TxnLog, type_arena::TypeArena,
    type_checker::TypeChecker, unification_too_complex::UnificationTooComplex,
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
    let ty = follow_type_id(ty);

    if let Some(ftv) = get_type_id::<FunctionType>(ty)
      && ftv.has_no_free_or_generic_types
    {
      return ty;
    }

    // reusableInstantiation.resetState(log, &currentModule->internalTypes, builtinTypes, scope->level, /*scope*/ nullptr);
    unsafe {
      let arena = &mut (*(Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module))
        .internal_types as *mut TypeArena;
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
