use core::ffi::c_char;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_rawgetfield::lua_rawgetfield, lua_toboolean::lua_toboolean, lua_type::lua_type},
  macros::lua_pop::lua_pop,
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn getboolfield(l: *mut lua_State, key: &str) -> i32 {
  let key_bytes = key.as_bytes();

  // lua_rawgetfield expects a null-terminated C string key.
  let mut buf = key_bytes.to_vec();
  buf.push(0);
  let key_c: *const c_char = buf.as_ptr() as *const c_char;

  unsafe {
    lua_rawgetfield(l, -1, key_c);

    // We cannot use the lua_isnil! macro because it calls the 0-arity lua_type stub directly,
    // which causes a compilation error. We manually implement the logic here.
    let is_nil = lua_type(l, -1) == (LuaType::Nil as i32);

    let res: i32 = if is_nil { -1 } else { lua_toboolean(l, -1) };

    lua_pop(l, 1);
    res
  }
}
