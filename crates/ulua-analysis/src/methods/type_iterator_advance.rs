//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Type.h:1209:type_iterator_advance`
//! Source: `Analysis/include/Luau/Type.h:1209-1222` (hand-ported)

use crate::records::type_iterator::{TypeIterator, TypeIteratorMember};

impl<T: TypeIteratorMember> TypeIterator<T> {
  pub(crate) fn advance(&mut self) {
    unsafe {
      while !self.stack.empty() {
        let (t, current_index) = self.stack.front_mut();
        *current_index += 1;

        let types = (**t).get_types();
        if *current_index >= types.len() {
          self.stack.pop_front();
        } else {
          break;
        }
      }
    }
  }
}
