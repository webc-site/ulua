//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Type.h:1157:type_iterator_operator_inc`
//! Source: `Analysis/include/Luau/Type.h:1157-1162` (hand-ported)

use crate::records::type_iterator::{TypeIterator, TypeIteratorMember};

impl<T: TypeIteratorMember> TypeIterator<T> {
  /// C++ `TypeIterator<T> operator++(int)` (post-increment).
  pub fn operator_inc_i32(&mut self) -> Self {
    let copy = self.clone();
    self.operator_inc();
    copy
  }
}
