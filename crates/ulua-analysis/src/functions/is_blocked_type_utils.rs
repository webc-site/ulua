use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::{follow_type, get_type},
  records::{
    blocked_type::BlockedType, pending_expansion_type::PendingExpansionType,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};

pub fn is_blocked(ty: TypeId) -> bool {
  let ty = follow_type::follow(ty);

  if let Some(tfit) = get_type::get::<TypeFunctionInstanceType>(ty)
    && tfit.state == TypeFunctionInstanceState::Unsolved
  {
    return true;
  }

  get_type::get::<BlockedType>(ty).is_some() || get_type::get::<PendingExpansionType>(ty).is_some()
}
