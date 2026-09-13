//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Type.h:1164:type_iterator_operator_eq`
//! Source: `Analysis/include/Luau/Type.h:1164-1170` (hand-ported)

use crate::records::type_iterator::{TypeIterator, TypeIteratorMember};

impl<T: TypeIteratorMember> TypeIterator<T> {
  pub fn operator_eq(&self, rhs: &Self) -> bool {
    if !self.stack.empty() && !rhs.stack.empty() {
      return *self.stack.front() == *rhs.stack.front();
    }

    self.stack.empty() && rhs.stack.empty()
  }
}
