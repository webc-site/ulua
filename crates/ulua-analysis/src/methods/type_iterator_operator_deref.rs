//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Type.h:1177:type_iterator_operator_deref`
//! Source: `Analysis/include/Luau/Type.h:1177-1193` (hand-ported)

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::follow_type::follow,
  records::type_iterator::{TypeIterator, TypeIteratorMember},
  type_aliases::type_id::TypeId,
};

impl<T: TypeIteratorMember> TypeIterator<T> {
  /// C++ `TypeId operator*()`.
  pub fn operator_deref(&mut self) -> TypeId {
    unsafe {
      self.descend();

      LUAU_ASSERT!(!self.stack.empty());

      let (t, current_index) = *self.stack.front();
      LUAU_ASSERT!(!t.is_null());

      let types = (*t).get_types();
      LUAU_ASSERT!(current_index < types.len());

      let ty = follow(types[current_index]);
      LUAU_ASSERT!(T::get_if(&(*ty).ty).is_none());

      ty
    }
  }
}
