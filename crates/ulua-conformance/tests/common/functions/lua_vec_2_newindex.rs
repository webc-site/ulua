use ulua_vm::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

use crate::common::functions::lua_vec_2_get::lua_vec_2_get;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vec_2_newindex(l: *mut LuaState) -> i32 {
  // Safety: `l` 为本用例存活的 LuaState；`lua_vec_2_get` 校验参数 1 为 Vec2 userdata
  // 并返回其数据指针。
  let v = unsafe { lua_vec_2_get(l, 1) };
  let name = unsafe { (*l).check_str(2) };
  // Safety: `l` 存活；`check_number` 在参数 3 为数字时返回其值（否则抛 Lua 错误）。
  let value = unsafe { (*l).check_number(3) as f32 };

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
