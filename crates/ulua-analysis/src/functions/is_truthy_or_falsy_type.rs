use crate::{
  enums::follow_option::FollowOption,
  functions::{
    follow_type::{FollowMapper, follow_full},
    is_approximately_falsy_type::is_approximately_falsy_type,
    is_approximately_truthy_type::is_approximately_truthy_type,
  },
  type_aliases::type_id::TypeId,
};
/// ty 的有效性由调用方按 C++ 同契约保证。
pub fn is_truthy_or_falsy_type(ty: TypeId) -> bool {
  let ty = follow_full(ty, FollowOption::Normal, FollowMapper::Identity);
  is_approximately_truthy_type(ty) || is_approximately_falsy_type(ty)
}
