use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::{isblack::isblack, isdead::isdead, iswhite::iswhite, keepinvariant::keepinvariant},
  records::{gc_object::GCObject, global_state::global_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn validateobjref(g: *mut global_State, f: *mut GCObject, t: *mut GCObject) {
  unsafe {
    LUAU_ASSERT!(!isdead!(g, t));

    if keepinvariant(g) {
      // 增量式基本不变量：黑色对象不可指向白色对象
      LUAU_ASSERT!(!(isblack!(f) && iswhite!(t)));
    }
  }
}
