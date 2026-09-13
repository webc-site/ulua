//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Type.h:1150:type_iterator_operator_inc`
//! Source: `Analysis/include/Luau/Type.h:1150-1155` (hand-ported)

use crate::records::type_iterator::{TypeIterator, TypeIteratorMember};

impl<T: TypeIteratorMember> TypeIterator<T> {
  /// C++ `TypeIterator<T>& operator++()`.
  pub fn operator_inc(&mut self) -> &mut Self {
    self.advance();
    self.descend();
    self
  }
}
