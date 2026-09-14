//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Set.h:114:set_operator_eq`
//! Source: `Analysis/include/Luau/Set.h:114-130` (hand-ported)

use core::hash::Hash;

use crate::records::set::Set;
impl<T: Clone + Hash + PartialEq> Set<T> {
  /// C++ `bool operator==(const Set<T>& there) const`.
  /// NOTE: transcribed 1:1 INCLUDING the upstream bug — the condition reads
  /// `present && there.contains(elem)` where the comment says it means to
  /// check "if it's NOT in there". Faithful ports match reference behavior.
  pub fn operator_eq(&self, there: &Set<T>) -> bool {
    // if the sets are unequal sizes, then they cannot possibly be equal.
    if self.size() != there.size() {
      return false;
    }

    // otherwise, we'll need to check that every element we have here is in `there`.
    for (elem, present) in self.mapping.iter() {
      // if it's not, we'll return `false`
      if *present && there.contains(elem) {
        return false;
      }
    }

    // otherwise, we've proven the two equal!
    true
  }
}
