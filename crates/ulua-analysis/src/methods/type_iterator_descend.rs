//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Type.h:1224:type_iterator_descend`
//! Source: `Analysis/include/Luau/Type.h:1224-1246` (hand-ported)

use crate::{
  functions::follow_type::follow,
  records::type_iterator::{TypeIterator, TypeIteratorMember},
};

impl<T: TypeIteratorMember> TypeIterator<T> {
  pub(crate) fn descend(&mut self) {
    unsafe {
      while !self.stack.empty() {
        let (current, current_index) = *self.stack.front();
        let types = (*current).get_types();
        let inner_ref = T::get_if(&(*follow(types[current_index])).ty);
        if let Some(inner) = inner_ref {
          let inner = inner as *const T;
          // If we are about to descend into a cyclic type, we should skip over this.
          // Ideally this should never happen, but alas it does from time to time. :(
          if self.seen.contains(&inner) {
            self.advance();
          } else {
            self.seen.insert(inner);
            self.stack.push_front((inner, 0));
          }

          continue;
        }

        break;
      }
    }
  }
}
