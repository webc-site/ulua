use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::get_type_alt_j::get_type_id,
  records::{
    blocked_type::BlockedType, pending_expansion_type::PendingExpansionType,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};

pub fn is_blocked_or_unsolved_type(ty: TypeId) -> bool {
  if let Some(tfit) = get_type_id::<TypeFunctionInstanceType>(ty)
    && tfit.state == TypeFunctionInstanceState::Unsolved
  {
    return true;
  }

  get_type_id::<BlockedType>(ty).is_some() || get_type_id::<PendingExpansionType>(ty).is_some()
}
