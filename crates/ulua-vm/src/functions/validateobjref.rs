use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::{isblack::isblack, isdead::isdead, iswhite::iswhite, keepinvariant::keepinvariant},
  records::{gc_object::GCObject, global_state::global_State},
};

/// # Safety
/// `g` 须指向存活 global_State；`f`/`t` 须为存活 GCObject 的地址（仅读其着色位，故 `*const` 足够）。
pub(crate) unsafe fn validateobjref(g: *mut global_State, f: *const GCObject, t: *const GCObject) {
  unsafe {
    LUAU_ASSERT!(!isdead!(g, t));

    if keepinvariant(&*g) {
      // 增量式基本不变量：黑色对象不可指向白色对象
      LUAU_ASSERT!(!(isblack!(f) && iswhite!(t)));
    }
  }
}
