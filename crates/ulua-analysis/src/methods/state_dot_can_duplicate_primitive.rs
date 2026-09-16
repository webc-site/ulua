use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    any_type::AnyType, never_type::NeverType, primitive_type::PrimitiveType, state_dot::StateDot,
    unknown_type::UnknownType,
  },
  type_aliases::{bound_type::BoundType, type_id::TypeId},
};

impl StateDot {
  pub fn can_duplicate_primitive(&self, ty: TypeId) -> bool {
    let bound = get_type_id::<BoundType>(ty);
    if !bound.is_none() {
      return false;
    }

    let primitive = get_type_id::<PrimitiveType>(ty);
    if !primitive.is_none() {
      return true;
    }

    let any = get_type_id::<AnyType>(ty);
    if !any.is_none() {
      return true;
    }

    let unknown = get_type_id::<UnknownType>(ty);
    if !unknown.is_none() {
      return true;
    }

    let never = get_type_id::<NeverType>(ty);
    !never.is_none()
  }
}
