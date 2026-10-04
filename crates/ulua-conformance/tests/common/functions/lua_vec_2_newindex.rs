use ulua_vm::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

use crate::common::functions::{lua_vec_2_get::lua_vec_2_get, safe_api::state_mut};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vec_2_newindex(l: *mut LuaState) -> i32 {
  // `lua_vec_2_get` 是 safe 门面：校验参数 1 为 Vec2 userdata 并返回其数据指针。
  let v = lua_vec_2_get(l, 1);
  let name = state_mut(l).check_str(2);
  // `check_number` 在参数 3 为数字时返回其值（否则抛 Lua 错误）。
  let value = state_mut(l).check_number(3) as f32;

  if name == "X" {
    // Safety: `v` 指向存活 Vec2 userdata 的数据，x 可写。
    unsafe { (*v).x = value };
  } else if name == "Y" {
    // Safety: `v` 指向存活 Vec2 userdata 的数据，y 可写。
    unsafe { (*v).y = value };
  } else {
    // 末分支按 cpp 抛 Lua 错误（格式串为已校验的 `name`），不返回。
    // Safety: `l` 存活；`luaL_error` 以 long-jump 终止本回调。
    unsafe { luaL_error!(&mut *l, "{name} is not a writable member of vec2") }
  }

  0
}
