use core::{ffi::c_char, ptr::null};

use crate::{macros::getstr::getstr, records::closure::Closure, type_aliases::proto::Proto};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn getfuncname(cl: *mut Closure) -> *const c_char {
  unsafe {
    if cl.is_null() {
      return null();
    }

    if (*cl).is_c != 0 {
      let c_debugname = (*cl).inner.c.debugname;
      if !c_debugname.is_null() {
        c_debugname
      } else {
        null()
      }
    } else {
      let p: *mut Proto = (*cl).inner.l.p;

      if !p.is_null() {
        let p_debugname = (&(*p)).debugname;
        if !p_debugname.is_null() {
          getstr(p_debugname)
        } else {
          null()
        }
      } else {
        null()
      }
    }
  }
}
