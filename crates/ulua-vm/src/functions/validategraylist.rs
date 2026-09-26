//! `validategraylist` — validate every node in a GC gray list.
//! C++ source: `VM/src/lgcdebug.cpp:218`
//!
//! Walks the singly-linked gray list starting at `o`; asserts each node is
//! still gray and follows the per-type `gclist` pointer to the next node.
//! Returns immediately if the GC invariant is not active (sweep phase etc.).

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::{isgray::isgray, keepinvariant::keepinvariant},
  records::{gc_object::GCObject, global_state::global_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn validategraylist(g: *mut global_State, mut o: *mut GCObject) {
  unsafe {
    if !keepinvariant(g) {
      return;
    }

    while !o.is_null() {
      LUAU_ASSERT!(isgray!(o));

      if let Some(next) = (*o).gclist() {
        o = next;
      } else {
        LUAU_ASSERT!(false);
        return;
      }
    }
  }
}
