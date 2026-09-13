use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{blocked_type::BlockedType, constraint::Constraint},
  type_aliases::type_id::TypeId,
};

pub fn can_mutate(ty: TypeId, constraint: *const Constraint) -> bool {
  let ty = follow_type_id(ty);
  if let Some(blocked) = get_type_id::<BlockedType>(ty) {
    let owner = blocked.get_owner();
    LUAU_ASSERT!(!owner.is_null());
    return owner == constraint;
  }

  true
}
