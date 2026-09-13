//! Source: `Analysis/src/Unifier.cpp` (Unifier::occursCheck(DenseHashSet<TypeId>&,...), L2644-2687)
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::get_mutable_type::get_mutable_type_id,
  records::{
    free_type::FreeType, intersection_type::IntersectionType, unifier::Unifier,
    union_type::UnionType,
  },
  type_aliases::{error_type::ErrorType, type_id::TypeId},
};

impl Unifier {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  /// `bool Unifier::occursCheck(DenseHashSet<TypeId>& seen, TypeId needle, TypeId haystack)`
  pub(crate) fn occurs_check_dense_hash_set_type_id_type_id_type_id(
    &mut self,
    seen: &mut DenseHashSet<TypeId>,
    mut needle: TypeId,
    mut haystack: TypeId,
  ) -> bool {
    let mut occurrence = false;

    needle = unsafe { self.log.follow_type_id(needle) };
    haystack = unsafe { self.log.follow_type_id(haystack) };

    if seen.find(&haystack).is_some() {
      return false;
    }

    seen.insert(haystack);

    if get_mutable_type_id::<ErrorType>(needle).is_some() {
      return false;
    }

    if get_mutable_type_id::<FreeType>(needle).is_none() {
      self.ice_string("Expected needle to be free");
    }

    if needle == haystack {
      return true;
    }

    if get_mutable_type_id::<FreeType>(haystack).is_some() {
      return false;
    } else if let Some(a) = get_mutable_type_id::<UnionType>(haystack) {
      let options = a.options.clone();
      for ty in options {
        if self.occurs_check_dense_hash_set_type_id_type_id_type_id(seen, needle, ty) {
          occurrence = true;
        }
      }
    } else if let Some(a) = get_mutable_type_id::<IntersectionType>(haystack) {
      let parts = a.parts.clone();
      for ty in parts {
        if self.occurs_check_dense_hash_set_type_id_type_id_type_id(seen, needle, ty) {
          occurrence = true;
        }
      }
    }

    occurrence
  }
}
