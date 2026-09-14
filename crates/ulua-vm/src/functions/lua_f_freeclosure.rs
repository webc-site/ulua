use core::ffi::c_int;

use crate::{
  functions::lua_m_freegco::luaM_freegco_,
  macros::{size_cclosure::size_cclosure, size_lclosure::size_lclosure},
  records::{closure::Closure, gc_object::GcObject, lua_page::lua_Page},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn lua_f_freeclosure(l: *mut lua_State, c: *mut Closure, page: *mut lua_Page) {
  unsafe {
    let size = if (*c).is_c != 0 {
      size_cclosure((*c).nupvalues as c_int)
    } else {
      size_lclosure((*c).nupvalues as usize)
    };

    luaM_freegco_(l, c as *mut GcObject, size, (*c).hdr.memcat, page);
  }
}
