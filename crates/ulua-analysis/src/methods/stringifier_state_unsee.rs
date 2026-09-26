//! Source: `Analysis/src/ToString.cpp:201-207` (hand-ported)
//!
//! C++ `void unsee(const void* tv)` — really erases (Luau::Set supports
//! erase). An earlier translation wired this to the DenseHashSet no-op
//! `unsee`, which made every repeated sibling type print `*CYCLE*`.

use core::ptr::from_ref;

use crate::records::stringifier_state::StringifierState;

impl StringifierState {
  /// §2：身份键入参收为共享引用，地址转换只作身份、不解引用。
  pub fn unsee<T>(&mut self, tv: &T) {
    let ttv = from_ref(tv).cast::<()>() as *mut ();

    if self.seen.contains(&ttv) {
      self.seen.erase(&ttv);
    }
  }
}
