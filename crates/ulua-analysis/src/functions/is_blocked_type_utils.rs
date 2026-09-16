use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    blocked_type::BlockedType, pending_expansion_type::PendingExpansionType,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};

pub fn is_blocked(ty: TypeId) -> bool {
  let ty = follow_type_id(ty);

  if let Some(tfit) = get_type_id::<TypeFunctionInstanceType>(ty)
    && tfit.state == TypeFunctionInstanceState::Unsolved
  {
    return true;
  }

  get_type_id::<BlockedType>(ty).is_some() || get_type_id::<PendingExpansionType>(ty).is_some()
}
