use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id, is_prim::is_prim},
  records::{any_type::AnyType, primitive_type::PrimitiveType, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

pub fn maybe_string(ty: TypeId) -> bool {
  let ty = follow_type_id(ty);

  if is_prim(ty, PrimitiveType::STRING) || get_type_id::<AnyType>(ty).is_some() {
    return true;
  }

  if let Some(utv) = get_type_id::<UnionType>(ty) {
    for &part in &utv.options {
      if maybe_string(part) {
        return true;
      }
    }
  }

  false
}
