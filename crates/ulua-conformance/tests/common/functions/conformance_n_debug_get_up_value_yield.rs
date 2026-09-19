use core::mem::zeroed;
use std::ffi::CStr;

use ulua_vm::{
  functions::{
    lua_checkstack::lua_checkstack, lua_getinfo::lua_getinfo, lua_getupvalue::lua_getupvalue,
  },
  macros::{lua_minstack::LUA_MINSTACK, lua_pop::lua_pop, lua_tointeger::lua_tointeger},
  records::{lua_debug::LuaDebug, lua_state::lua_State},
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_n_debug_get_up_value_yield(l: *mut lua_State) -> bool {
  unsafe {
    lua_checkstack(l, LUA_MINSTACK);

    let mut ar: LuaDebug = zeroed();
    assert_ne!(0, lua_getinfo(l, 1, c"f".as_ptr(), &mut ar));

    let upvalue = lua_getupvalue(l, -1, 1);
    assert!(!upvalue.is_null());
    assert_eq!(CStr::from_ptr(upvalue).to_bytes(), b"");
    assert_eq!(lua_tointeger!(l, -1), 5);
    lua_pop(l, 2);

    false
  }
}
