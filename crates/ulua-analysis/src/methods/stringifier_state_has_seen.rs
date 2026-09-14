//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:191:stringifier_state_has_seen`
//! Source: `Analysis/src/ToString.cpp:191-199` (hand-ported)

use core::ffi::c_void;

use crate::records::stringifier_state::StringifierState;

impl StringifierState {
  /// C++ `bool hasSeen(const void* tv)`.
  pub fn has_seen(&mut self, tv: *const c_void) -> bool {
    let ttv = tv as *mut c_void;
    if self.seen.contains(&ttv) {
      return true;
    }

    self.seen.insert(&ttv);
    false
  }
}
