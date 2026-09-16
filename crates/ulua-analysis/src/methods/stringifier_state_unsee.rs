//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:201:stringifier_state_unsee`
//! Source: `Analysis/src/ToString.cpp:201-207` (hand-ported)
//!
//! C++ `void unsee(const void* tv)` — really erases (Luau::Set supports
//! erase). An earlier translation wired this to the DenseHashSet no-op
//! `unsee`, which made every repeated sibling type print `*CYCLE*`.

use core::ffi::c_void;

use crate::records::stringifier_state::StringifierState;

impl StringifierState {
  pub fn unsee(&mut self, tv: *const c_void) {
    let ttv = tv as *mut c_void;

    if self.seen.contains(&ttv) {
      self.seen.erase(&ttv);
    }
  }
}
