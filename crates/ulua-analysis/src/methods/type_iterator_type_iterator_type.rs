//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Type.h:1138:type_iterator_type_iterator`
//! Source: `Analysis/include/Luau/Type.h:1138-1148` (hand-ported)

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::type_iterator::{TypeIterator, TypeIteratorMember};

impl<T: TypeIteratorMember> TypeIterator<T> {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
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
