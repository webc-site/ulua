//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Type.h:1130:type_iterator`
//! Source: `Analysis/include/Luau/Type.h:1126-1248` (hand-ported)
//!
//! C++ `template<typename T> struct TypeIterator` — traverses T (UnionType or
//! IntersectionType) yielding each TypeId; encountering a nested T yields the
//! TypeIds within instead. (An earlier translation hardcoded UnionType in
//! `advance`, silently breaking intersection iteration.)

/// C++ `getTypes(const T*)` overload pair (Type.cpp).
use alloc::vec::Vec;

use ulua_common::records::{dense_hash_set::DenseHashSet, vec_deque::VecDeque};

use crate::{
  records::{intersection_type::IntersectionType, union_type::UnionType},
  type_aliases::{type_id::TypeId, type_variant::TypeVariantMember},
};
pub trait TypeIteratorMember: TypeVariantMember + 'static {
  fn get_types(&self) -> &Vec<TypeId>;
}

impl TypeIteratorMember for UnionType {
  fn get_types(&self) -> &Vec<TypeId> {
    &self.options
  }
}

impl TypeIteratorMember for IntersectionType {
  fn get_types(&self) -> &Vec<TypeId> {
    &self.parts
  }
}

#[derive(Debug)]
pub struct TypeIterator<T: TypeIteratorMember> {
  // (const T* t, size_t current_index)
  pub(crate) stack: VecDeque<(*const T, usize)>,
  /// Only needed to protect the iterator from hanging the thread.
  pub(crate) seen: DenseHashSet<*const T>,
}

// Manual impl: the derive would demand `T: Clone`, but the stack stores
// `*const T` (Copy) — the element type itself is never cloned.
impl<T: TypeIteratorMember> Clone for TypeIterator<T> {
  fn clone(&self) -> Self {
    Self {
      stack: self.stack.clone(),
      seen: self.seen.clone(),
    }
  }
}
