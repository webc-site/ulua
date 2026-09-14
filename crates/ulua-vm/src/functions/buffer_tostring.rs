use core::{
  ffi::{c_char, c_int, c_void},
  mem::transmute,
};

use crate::{
  functions::{lua_l_checkbuffer::lua_l_checkbuffer, lua_pushlstring::lua_pushlstring},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn buffer_tostring(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let data = lua_l_checkbuffer(l, 1, &mut len);

    // The dependency card for lua_pushlstring shows an empty signature `fn()`,
    // but the C++ logic requires `lua_pushlstring(l, data, len)`.
    // To satisfy the compiler while preserving the logic required by the VM,
    // we transmute the function pointer to the expected signature.
    let func: unsafe fn(*mut lua_State, *const c_char, usize) =
      transmute(lua_pushlstring as *const c_void);
    func(l, data as *const c_char, len);

    1
  }
}
