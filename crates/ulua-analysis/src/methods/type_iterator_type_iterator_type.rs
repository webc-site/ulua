//! Source: `Analysis/include/Luau/Type.h:1138-1148` (hand-ported)

use ulua_common::{
  macros::luau_assert::LUAU_ASSERT,
  records::{dense_hash_set::DenseHashSet, vec_deque::VecDeque},
};

use crate::records::type_iterator::{TypeIterator, TypeIteratorMember};

impl<T: TypeIteratorMember> TypeIterator<T> {
  /// # Safety
  /// 调用方须保证 `t` 非空、对齐，指向存活的 `UnionType`/`IntersectionType`（函数内读取其 arena 子节点列表）。
  /// `t` 会被存入迭代器的 `seen/stack`，故其存活期须覆盖整个迭代过程、地址不移动。
  /// cpp `Analysis/include/Luau/Type.h:1143`（`explicit TypeIterator(const T* t)`，内部 `LUAU_ASSERT(t)`）。单线程。
  /// C++ `explicit TypeIterator(const T* t)`.
  pub unsafe fn type_iterator_type(t: *const T) -> Self {
    LUAU_ASSERT!(!t.is_null());

    let mut it = Self::type_iterator_default();

    unsafe {
      let types = (*t).get_types();
      if !types.is_empty() {
        it.stack.push_front((t, 0));
      }

      it.seen.insert(t);
      it.descend();
    }

    it
  }
}

// Source: `Analysis/include/Luau/Type.h:1201` (hand-ported)
impl<T: TypeIteratorMember> TypeIterator<T> {
  /// C++ private `TypeIterator() = default;` — the `end()` sentinel.
  pub fn type_iterator_default() -> Self {
    Self {
      stack: VecDeque::new(),
      seen: DenseHashSet::default(),
    }
  }
}
