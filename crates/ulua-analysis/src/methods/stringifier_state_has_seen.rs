//! Source: `Analysis/src/ToString.cpp:191-199` (hand-ported)

use core::ptr::from_ref;

use crate::records::stringifier_state::StringifierState;

impl StringifierState {
  /// C++ `bool hasSeen(const void* tv)`. §2：身份键入参收为共享引用，
  /// 地址转换是本函数唯一指针操作（只作身份，从不解引用该指针）。
  pub fn has_seen<T>(&mut self, tv: &T) -> bool {
    let ttv = from_ref(tv).cast::<()>() as *mut ();
    if self.seen.contains(&ttv) {
      return true;
    }

    self.seen.insert(&ttv);
    false
  }
}
