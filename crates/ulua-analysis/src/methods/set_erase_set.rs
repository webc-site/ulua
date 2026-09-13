//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Set.h:55:set_erase`
//! Source: `Analysis/include/Luau/Set.h:55-65` (hand-ported)

use core::hash::Hash;

use crate::records::set::Set;
impl<T: Clone + Hash + PartialEq> Set<T> {
  /// C++ `void erase(T&& element)` / `void erase(const T& element)` —
  /// tombstones the entry (sets it false) rather than removing the slot.
  pub fn erase(&mut self, element: &T) {
    let entry = self.mapping.get_or_insert(element.clone());

    if *entry {
      *entry = false;
      self.entry_count -= 1;
    }
  }
}
