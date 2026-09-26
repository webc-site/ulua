use core::ptr::null_mut;

use ulua_vm::{
  functions::lua_namecallatom::lua_namecallatom,
  macros::{lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

use crate::common::functions::{
  cstr_text::cstr_text, lua_vertex_clone::lua_vertex_clone, lua_vertex_get::lua_vertex_get,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vertex_namecall(l: *mut LuaState) -> i32 {
  // Safety: `l` 为本用例存活的 LuaState；`lua_namecallatom` 返回 namecall 方法名的
  // NUL 结尾指针或 null（第二实参 null_mut() 表示不取 arg 计数）。
  // FFI: c-API 要求 NULL
  let str_ptr = unsafe { lua_namecallatom(l, null_mut()) };

  if !str_ptr.is_null() {
    // `lua_vertex_get` 是 safe 门面：校验参数 1 为 Vertex userdata 并返回其数据指针。
    let self_ptr = lua_vertex_get(l, 1);
    // Safety: 上面已排除 null，`lua_namecallatom` 保证其 NUL 结尾。
    let str_slice = unsafe { cstr_text(str_ptr) };

    // Clone 是 safe 门面（自行压栈并给出返回计数）。
    if str_slice == "Clone" {
      return lua_vertex_clone(l, self_ptr);
    }
  }

  // Safety: `l` 存活；参数 1 为串时宏返回 NUL 结尾缓冲（否则抛 Lua 错误）。
  let arg1_ptr = unsafe { luaL_checkstring!(l, 1) };
  // Safety: 上一行保证 `arg1_ptr` 为 NUL 结尾串。
  let arg1_str = unsafe { cstr_text(arg1_ptr.cast()) };

  // Safety: 末分支按 cpp 抛「非方法」Lua 错误（宏内为 C ABI `luaL_error`，`l` 存活、
  // 格式串为已校验的 `arg1_str`）；该调用不返回。
  unsafe { luaL_error!(l, "{} is not a valid method of vertex", arg1_str) }
}
