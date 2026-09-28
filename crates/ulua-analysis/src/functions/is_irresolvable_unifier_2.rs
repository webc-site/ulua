use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::{get_type, get_type_pack},
  records::{
    blocked_type::BlockedType, blocked_type_pack::BlockedTypePack,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub fn is_irresolvable(ty: TypeId) -> bool {
  let tfit = get_type::get::<TypeFunctionInstanceType>(ty);
  if let Some(tfit) = tfit
    && tfit.state != TypeFunctionInstanceState::Unsolved
  {
    return false;
  }
  get_type::get::<BlockedType>(ty).is_some() || tfit.is_some()
}

// returns `true` if `tp` is irresolvable and should be added to `incompleteSubtypes`.
pub fn is_irresolvable_type_pack(tp: TypePackId) -> bool {
  get_type_pack::get::<BlockedTypePack>(tp).is_some()
    || get_type_pack::get::<TypeFunctionInstanceTypePack>(tp).is_some()
}
