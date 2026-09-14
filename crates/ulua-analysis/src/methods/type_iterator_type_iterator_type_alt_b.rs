//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Type.h:1201:type_iterator_type_iterator`
//! Source: `Analysis/include/Luau/Type.h:1201` (hand-ported)

use core::ptr::null;

use ulua_common::records::{dense_hash_set::DenseHashSet, vec_deque::VecDeque};

use crate::records::type_iterator::{TypeIterator, TypeIteratorMember};
impl<T: TypeIteratorMember> TypeIterator<T> {
  /// C++ private `TypeIterator() = default;` — the `end()` sentinel.
  pub fn type_iterator_default() -> Self {
    Self {
      stack: VecDeque::new(),
      seen: DenseHashSet::new(null()),
    }
  }
}
