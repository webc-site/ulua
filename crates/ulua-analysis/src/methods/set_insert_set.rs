//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Set.h:34:set_insert`
//! Source: `Analysis/include/Luau/Set.h:34-47` (hand-ported)

use core::hash::Hash;

use crate::records::set::Set;
impl<T: Clone + Hash + PartialEq> Set<T> {
  /// C++ `bool insert(const T& element)` — true when newly inserted.
  pub fn insert(&mut self, element: &T) -> bool {
    let entry = self.mapping.get_or_insert(element.clone());
    let fresh = !*entry;

    if fresh {
      *entry = true;
      self.entry_count += 1;
    }

    fresh
  }
}
