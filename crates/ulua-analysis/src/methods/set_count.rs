//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Set.h:93:set_count`
//! Source: `Analysis/include/Luau/Set.h:93-97` (hand-ported)

use core::hash::Hash;

use crate::records::set::Set;
impl<T: Clone + Hash + PartialEq> Set<T> {
  /// C++ `size_t count(const T& element) const`.
  pub fn count(&self, element: &T) -> usize {
    match self.mapping.find(element) {
      Some(entry) if *entry => 1,
      _ => 0,
    }
  }
}
