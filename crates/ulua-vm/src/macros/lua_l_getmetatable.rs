use core::{
  ffi::{c_char, c_int, c_void},
  mem::transmute,
};

use crate::{
  functions::lua_getfield::lua_getfield, macros::lua_registryindex::LUA_REGISTRYINDEX,
  records::lua_state::lua_State,
};

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
/// `n` must be a valid pointer to a null-terminated C string.
#[inline(always)]
pub unsafe fn lua_l_getmetatable(l: *mut lua_State, n: *const c_char) -> c_int {
  unsafe {
    // The dependency lua_getfield is currently a stub in the required context (pub fn lua_getfield();).
    // However, the C++ source and the call site require it to take (l, idx, k) and return int.
    // We must cast the function pointer or call it as it is defined in the actual VM implementation.
    // Since we cannot change the signature of the dependency here, we use a transmute to call it with the correct signature.
    let func: unsafe fn(*mut lua_State, c_int, *const c_char) -> c_int =
      transmute(lua_getfield as *const c_void);
    func(l, LUA_REGISTRYINDEX, n)
  }
}

pub use lua_l_getmetatable as luaL_getmetatable;
