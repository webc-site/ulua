use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::{follow_type, follow_type_pack, get_type, get_type_pack},
  records::{
    blocked_type::BlockedType, blocked_type_pack::BlockedTypePack,
    constraint_solver::ConstraintSolver, pending_expansion_type::PendingExpansionType,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ConstraintSolver {
  pub fn is_blocked_type_id(&self, ty: TypeId) -> bool {
    // FIXME CLI-180636: Eventually this should use the same logic as
    // `SubtypingUnifier`, which is that blocked types are only based
    // on their type and any additional state, rather than looking at
    // `uninhabitedTypeFunctions`.
    let ty = follow_type::follow(ty);

    if let Some(tfit) = get_type::get::<TypeFunctionInstanceType>(ty) {
      if tfit.state != TypeFunctionInstanceState::Unsolved {
        return false;
      }

      return !self.uninhabited_type_functions.contains(&(ty as *const ()));
    }

    get_type::get::<BlockedType>(ty).is_some()
      || get_type::get::<PendingExpansionType>(ty).is_some()
  }

  pub(crate) fn is_blocked_type_pack_id(&self, tp: TypePackId) -> bool {
    let tp = follow_type_pack::follow(tp);

    if get_type_pack::get::<TypeFunctionInstanceTypePack>(tp).is_some() {
      return !self.uninhabited_type_functions.contains(&(tp as *const ()));
    }

    get_type_pack::get::<BlockedTypePack>(tp).is_some()
  }
}
