//! Source: `VM/src/lgc.cpp`

use core::ptr::addr_of_mut;

/// Returns non-zero if the GC object `o` has been cleared (collected).
///
/// Strings are treated as values and are never considered cleared: their white
/// bits are reset so they will not be swept, and 0 is returned immediately.
/// For all other collectable types the function returns the `iswhite` result —
/// non-zero means the object's white bits are still set, i.e. it was not
/// reached during the mark phase and has been (or will be) collected.
///
/// C++ original: `static int isobjcleared(GCObject* o)` in VM/src/lgc.cpp:608
use crate::{
  enums::lua_type::LuaType,
  macros::stringmark::stringmark,
  records::{gc_object::GCObject, t_string::tstring},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
pub(crate) unsafe fn isobjcleared(o: *mut GCObject) -> i32 {
  unsafe {
    if (*o).gch.tt == LuaType::String as u8 {
      // strings are 'values', so they are never weak — stringmark(&o->ts)
      stringmark!(addr_of_mut!((*o).ts) as *mut tstring);
      0
    } else {
      // iswhite(o): test WHITE0BIT (bit 0) and WHITE1BIT (bit 1)
      // Using inline bit test because the iswhite! macro has a type-mismatch
      // (testbits takes i32 but marked is u8, and WHITE0BIT/WHITE1BIT are not
      // re-exported from crate::macros in this workspace build).
      ((*o).gch.marked & 0b0000_0011_u8) as i32
    }
  }
}
