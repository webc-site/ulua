//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Set.h:104:set_iteration`
//! Source: `Analysis/include/Luau/Set.h:104-199` (hand-ported)

use core::hash::Hash;

use crate::records::set::Set;
impl<T: Clone + Hash + PartialEq> Set<T> {
  /// C++ `begin()/end()` over live entries only. `erase` leaves tombstones in
  /// the underlying map, so iteration must skip entries whose presence bit is
  /// false.
  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self
      .mapping
      .iter()
      .filter_map(|(element, present)| if *present { Some(element) } else { None })
  }
}
