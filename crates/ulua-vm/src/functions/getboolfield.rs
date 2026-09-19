use core::ffi::CStr;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_rawgetfield::lua_rawgetfield, lua_toboolean::lua_toboolean, lua_type::lua_type},
  macros::lua_pop::lua_pop,
  type_aliases::lua_state::lua_State,
};

/// `key` 由 `&CStr` 承担 NUL 结尾契约（调用点为 `c"..."` 字面量，零分配）。
///
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn getboolfield(l: *mut lua_State, key: &CStr) -> i32 {
  unsafe {
    lua_rawgetfield(l, -1, key.as_ptr());

    // We cannot use the lua_isnil! macro because it calls the 0-arity lua_type stub directly,
    // which causes a compilation error. We manually implement the logic here.
    let is_nil = lua_type(l, -1) == (LuaType::Nil as i32);

    let res: i32 = if is_nil { -1 } else { lua_toboolean(l, -1) };

    lua_pop(l, 1);
    res
  }
}
