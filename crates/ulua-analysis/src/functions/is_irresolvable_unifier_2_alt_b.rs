use crate::{
  functions::get_type_pack::get_type_pack_id,
  records::{
    blocked_type_pack::BlockedTypePack,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};

// returns `true` if `tp` is irresolvable and should be added to `incompleteSubtypes`.
pub fn is_irresolvable(tp: TypePackId) -> bool {
  !get_type_pack_id::<BlockedTypePack>(tp).is_none()
    || !get_type_pack_id::<TypeFunctionInstanceTypePack>(tp).is_none()
}
