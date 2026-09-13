//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Set.h:83:set_size`
//! Source: `Analysis/include/Luau/Set.h:83-86` (hand-ported)

use core::hash::Hash;

use crate::records::set::Set;
impl<T: Clone + Hash + PartialEq> Set<T> {
  pub fn size(&self) -> usize {
    self.entry_count
  }
}
