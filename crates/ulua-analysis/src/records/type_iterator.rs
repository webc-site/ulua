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

impl<T: TypeIteratorMember> PartialEq for TypeIterator<T> {
  fn eq(&self, rhs: &Self) -> bool {
    if !self.stack.empty() && !rhs.stack.empty() {
      return *self.stack.front() == *rhs.stack.front();
    }
    self.stack.empty() && rhs.stack.empty()
  }
}

impl<T: TypeIteratorMember> Eq for TypeIterator<T> {}

/// 标准迭代器适配：等价于 C++ 的 `it != end; *it; ++it` 三段式循环，
/// 使调用点可以写 `for part in begin_union_type(ut)`。
impl<T: TypeIteratorMember> Iterator for TypeIterator<T> {
  type Item = TypeId;

  fn next(&mut self) -> Option<TypeId> {
    if self.stack.empty() {
      return None;
    }
    self.descend();
    if self.stack.empty() {
      return None;
    }
    let ty = self.current();
    self.advance();
    Some(ty)
  }
}
