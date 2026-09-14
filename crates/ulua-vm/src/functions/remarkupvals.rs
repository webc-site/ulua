//! Node: `cxx:Function:Luau.VM:VM/src/lgc.cpp:839:remarkupvals`
//! Source: `VM/src/lgc.cpp` (lgc.cpp:839-856, hand-ported)

use core::mem::size_of;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::{isblack::isblack, isgray::isgray, markvalue::markvalue, upisopen::upisopen},
  records::{gc_object::GCObject, global_state::global_State, up_val::UpVal},
};

pub(crate) unsafe fn remarkupvals(g: *mut global_State) -> usize {
  unsafe {
    let mut work: usize = 0;

    let uvhead = core::ptr::addr_of_mut!((*g).uvhead);
    let mut uv = (*g).uvhead.u.open.next;
    while uv != uvhead {
      work += size_of::<UpVal>();

      LUAU_ASSERT!(upisopen!(uv));
      LUAU_ASSERT!(
        (*(*uv).u.open.next).u.open.prev == uv && (*(*uv).u.open.prev).u.open.next == uv
      );
      // open upvalues are never black
      LUAU_ASSERT!(!isblack!(uv as *mut GCObject));

      if isgray!(uv as *mut GCObject) {
        markvalue!(g, (*uv).v);
      }

      uv = (*uv).u.open.next;
    }

    work
  }
}
