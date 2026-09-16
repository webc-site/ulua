use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::get_type_alt_j::get_type_id,
  records::{blocked_type::BlockedType, type_function_instance_type::TypeFunctionInstanceType},
  type_aliases::type_id::TypeId,
};

pub fn is_irresolvable(ty: TypeId) -> bool {
  let tfit = get_type_id::<TypeFunctionInstanceType>(ty);
  if let Some(tfit) = tfit
    && tfit.state != TypeFunctionInstanceState::Unsolved
  {
    return false;
  }
  get_type_id::<BlockedType>(ty).is_some() || tfit.is_some()
}
