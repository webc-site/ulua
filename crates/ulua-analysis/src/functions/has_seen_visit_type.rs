//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/VisitType.h:36:has_seen`
//! Source: `Analysis/include/Luau/VisitType.h:36-40` (hand-ported)
//!
//! C++ `bool hasSeen(std::unordered_set<void*>& seen, const void* tv)` — the
//! forgetting-set overload used by `TypeVisitor`. (An earlier translation
//! wrongly took `DenseHashSet`, silently giving TypeVisitor visit-once
//! semantics.)

use core::ffi::c_void;
use std::collections::HashSet;
pub fn has_seen(seen: &mut HashSet<*mut c_void>, tv: *const c_void) -> bool {
  let ttv = tv as *mut c_void;
  !seen.insert(ttv)
}
