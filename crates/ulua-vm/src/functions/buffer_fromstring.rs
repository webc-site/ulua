use core::{ffi::c_int, ptr::copy_nonoverlapping};

use crate::{
  functions::{lua_l_checklstring::lua_l_checklstring, lua_newbuffer::lua_newbuffer},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn buffer_fromstring(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let val = lua_l_checklstring(l, 1, &mut len);

    let data = lua_newbuffer(l, len);
    copy_nonoverlapping(val as *const u8, data as *mut u8, len);

    1
  }
}
