//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Type.h:1172:type_iterator_operator_ne`
//! Source: `Analysis/include/Luau/Type.h:1172-1175` (hand-ported)

use crate::records::type_iterator::{TypeIterator, TypeIteratorMember};

impl<T: TypeIteratorMember> TypeIterator<T> {
  pub fn operator_ne(&self, rhs: &Self) -> bool {
    !self.operator_eq(rhs)
  }
}
