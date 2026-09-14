use core::{ffi::c_char, ptr::copy_nonoverlapping};

use crate::{functions::extendstrbuf::extendstrbuf, type_aliases::lua_l_strbuf::LuaLStrbuf};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn lua_l_addlstring(b: *mut LuaLStrbuf, s: *const c_char, len: usize) {
  unsafe {
    let current_buffer_size = (*b).end.offset_from((*b).p) as usize;
    if current_buffer_size < len {
      extendstrbuf(b, len - current_buffer_size, -1);
    }

    copy_nonoverlapping(s as *const u8, (*b).p as *mut u8, len);
    (*b).p = (*b).p.add(len);
  }
}
