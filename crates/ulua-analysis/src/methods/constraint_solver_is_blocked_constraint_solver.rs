use core::ffi::c_void;

use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    blocked_type::BlockedType, constraint_solver::ConstraintSolver,
    pending_expansion_type::PendingExpansionType,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};
impl ConstraintSolver {
  pub fn is_blocked_type_id(&self, ty: TypeId) -> bool {
    // FIXME CLI-180636: Eventually this should use the same logic as
    // `SubtypingUnifier`, which is that blocked types are only based
    // on their type and any additional state, rather than looking at
    // `uninhabitedTypeFunctions`.
    let ty = follow_type_id(ty);

    if let Some(tfit) = get_type_id::<TypeFunctionInstanceType>(ty) {
      if tfit.state != TypeFunctionInstanceState::Unsolved {
        return false;
      }

      return !self
        .uninhabited_type_functions
        .contains(&(ty as *const c_void));
    }

    get_type_id::<BlockedType>(ty).is_some() || get_type_id::<PendingExpansionType>(ty).is_some()
  }
}
