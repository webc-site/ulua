use std::ptr::from_ref;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id, is_prim::is_prim},
  records::{
    primitive_type::PrimitiveType, singleton_type::SingletonType,
    string_singleton::StringSingleton, type_iterator::TypeIterator, union_type::UnionType,
  },
  type_aliases::{singleton_variant::SingletonVariantMember, type_id::TypeId},
};
pub fn is_string(ty: TypeId) -> bool {
  let ty = follow_type_id(ty);

  if is_prim(ty, PrimitiveType::STRING) {
    return true;
  }

  if let Some(st) = get_type_id::<SingletonType>(ty)
    && StringSingleton::get_if(&st.variant).is_some()
  {
    return true;
  }

  if let Some(utv) = get_type_id::<UnionType>(ty) {
    // C++ `std::all_of(begin(utv), end(utv), isString)` — iterate via the
    // UnionTypeIterator, which follows + flattens nested unions and carries
    // a seen-set to skip cycles. A raw recursion over `utv.options` stack-
    // overflows on a structurally self-referential union.
    let mut it = unsafe { TypeIterator::<UnionType>::type_iterator_type(from_ref(utv)) };
    let end_it = TypeIterator::<UnionType>::type_iterator_default();
    while it.operator_ne(&end_it) {
      let option = it.operator_deref();
      it.operator_inc();
      if !is_string(option) {
        return false;
      }
    }
    return true;
  }

  false
}
