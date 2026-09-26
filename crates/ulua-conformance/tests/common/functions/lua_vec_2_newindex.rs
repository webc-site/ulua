use ulua_vm::{
  functions::lua_l_checknumber::lua_l_checknumber,
  macros::{lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

use crate::common::functions::{cstr_text::cstr_text, lua_vec_2_get::lua_vec_2_get};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vec_2_newindex(l: *mut LuaState) -> i32 {
  // Safety: `l` 为本用例存活的 LuaState；`lua_vec_2_get` 校验参数 1 为 Vec2 userdata
  // 并返回其数据指针。
  let v = unsafe { lua_vec_2_get(l, 1) };
  // Safety: `luaL_checkstring!` 在参数 2 为串时返回 NUL 结尾缓冲（否则抛 Lua 错误）。
  let name_ptr = unsafe { luaL_checkstring!(l, 2) };
  // Safety: 上一行的宏契约保证 `name_ptr` 为 NUL 结尾合法 C 串。
  let name = unsafe { cstr_text(name_ptr.cast()) };
  // Safety: `l` 存活；`lua_l_checknumber` 在参数 3 为数字时返回其值（否则抛 Lua 错误）。
  let value = unsafe { lua_l_checknumber(l, 3) } as f32;

  if name == "X" {
    // Safety: `v` 指向存活 Vec2 userdata 的数据，x 可写。
    unsafe { (*v).x = value };
  } else if name == "Y" {
    // Safety: `v` 指向存活 Vec2 userdata 的数据，y 可写。
    unsafe { (*v).y = value };
  } else {
    // Safety: 末分支按 cpp 抛 Lua 错误（`l` 存活、格式串为已校验的 `name`），不返回。
    unsafe { luaL_error!(l, "{name} is not a writable member of vec2") }
  }

  0
}
