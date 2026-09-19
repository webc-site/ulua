use core::ffi::c_void;

use crate::{enums::lua_type::LuaType, macros::gco_2_u::gco2u, records::gc_object::GCObject};

#[inline]
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn enumtopointer(gco: *mut GCObject) -> *mut c_void {
  unsafe {
    if (*gco).gch.tt == (LuaType::UserData as u8) {
      let u = gco2u!(gco);
      (*u).data.as_mut_ptr() as *mut c_void
    } else {
      gco as *mut c_void
    }
  }
}
