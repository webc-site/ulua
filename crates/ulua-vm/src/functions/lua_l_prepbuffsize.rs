use core::ffi::c_char;

use crate::{functions::extendstrbuf::extendstrbuf, records::lua_l_strbuf::LuaLStrbuf};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn lua_l_prepbuffsize(b: *mut LuaLStrbuf, size: usize) -> *mut c_char {
  unsafe {
    let current_p = (*b).p;
    let current_end = (*b).end;
    if (current_end as usize).wrapping_sub(current_p as usize) < size {
      extendstrbuf(
        b,
        size.wrapping_sub((current_end as usize).wrapping_sub(current_p as usize)),
        -1,
      )
    } else {
      current_p
    }
  }
}
