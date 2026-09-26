use core::ptr::null_mut;

use ulua_vm::{
  functions::lua_namecallatom::lua_namecallatom,
  macros::{lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

use crate::common::functions::{
  cstr_text::cstr_text, lua_vec_2_clone::lua_vec_2_clone, lua_vec_2_dot::lua_vec_2_dot,
  lua_vec_2_get::lua_vec_2_get, lua_vec_2_min::lua_vec_2_min, lua_vec_2_reenter::lua_vec_2_reenter,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vec_2_namecall(l: *mut LuaState) -> i32 {
  // Safety: `l` 为本用例存活的 LuaState；`lua_namecallatom` 返回 namecall 原子的
  // NUL 结尾串指针或 null（第二实参 null_mut() 表示不取 arg 计数）。
  // FFI: c-API 要求 NULL
  let str_ptr = unsafe { lua_namecallatom(l, null_mut()) };

  if !str_ptr.is_null() {
    // Safety: 上一步保证 `str_ptr` 非空且 NUL 结尾。
    let str_slice = unsafe { cstr_text(str_ptr) };

    // 分派：`lua_vec_2_get` 校验 self 为 Vec2 userdata 并交回数据指针；下游
    // Dot/Min/Clone/Reenter 都是 safe 门面（自行压栈并给出返回计数）。
    if str_slice == "Dot" {
      // Safety: `l` 存活，参数 1 为 Vec2 userdata（否则按 cpp 抛 Lua 错误）。
      let self_ptr = unsafe { lua_vec_2_get(l, 1) };
      return lua_vec_2_dot(l, self_ptr);
    }

    if str_slice == "Min" {
      // Safety: 同上——`l` 存活，参数 1 为 Vec2 userdata。
      let self_ptr = unsafe { lua_vec_2_get(l, 1) };
      return lua_vec_2_min(l, self_ptr);
    }

    if str_slice == "Clone" {
      // Safety: 同上——`l` 存活，参数 1 为 Vec2 userdata。
      let self_ptr = unsafe { lua_vec_2_get(l, 1) };
      return lua_vec_2_clone(l, self_ptr);
    }

    if str_slice == "Reenter" {
      // Safety: 同上——`l` 存活，参数 1 为 Vec2 userdata。
      let self_ptr = unsafe { lua_vec_2_get(l, 1) };
      return lua_vec_2_reenter(l, self_ptr);
    }
  }

  // Safety: `l` 存活；参数 1 为串时宏返回 NUL 结尾缓冲（否则抛 Lua 错误）。
  let arg1_ptr = unsafe { luaL_checkstring!(l, 1) };
  // Safety: 上一行的宏契约保证 `arg1_ptr` 指向 NUL 结尾串。
  let arg1_str = unsafe { cstr_text(arg1_ptr.cast()) };

  // Safety: 末分支按 cpp 抛 Lua 错误（宏内为 C ABI `luaL_error`，`l` 存活、格式串为
  // 已校验的 `arg1_str`）；该调用不返回。
  unsafe { luaL_error!(l, "{} is not a valid method of vector", arg1_str) }
}
