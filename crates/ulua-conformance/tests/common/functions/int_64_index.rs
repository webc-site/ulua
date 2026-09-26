use core::ffi::c_int;

use ulua_vm::{
  functions::{lua_pushnumber::lua_pushnumber, lua_touserdatatagged::lua_touserdatatagged},
  macros::{
    lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error, lua_l_typeerror::luaL_typeerror,
  },
  records::lua_state::LuaState,
};

use crate::common::functions::{
  cstr_text::{cstr_raw, cstr_text},
  k_int_64_tag::K_INT_64_TAG,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn int_64_index(l: *mut LuaState) -> c_int {
  // Safety: `l` 为本用例存活的 LuaState；按 tag 取参数 1 的 userdata 数据指针，
  // tag 不匹配时返回 null（下一行即转类型错误）。
  let p = unsafe { lua_touserdatatagged(l, 1, K_INT_64_TAG) };
  if p.is_null() {
    // Safety: 按 cpp 以「不是 int64」抛类型错误（宏内为 C ABI `luaL_typeerror`），不返回。
    unsafe { luaL_typeerror!(l, 1, "int64") };
  }

  // Safety: `l` 存活；参数 2 为串时宏返回 NUL 结尾缓冲（否则抛 Lua 错误）。
  let name_ptr = unsafe { luaL_checkstring!(l, 2) };
  // Safety: 上一行保证 `name_ptr` 为 NUL 结尾合法 C 串。
  let name = unsafe { cstr_raw(name_ptr) };

  if name == b"value" {
    // Safety: 上面已排除 null，`p` 即 tag 为 K_INT_64_TAG 的 userdata 数据区，可读 i64；
    // `l` 存活，压入其 f64 视图。
    unsafe { lua_pushnumber(l, *(p as *const i64) as f64) };
    return 1;
  }

  // Safety: `name_ptr` 为 NUL 结尾串（lossy 渲染仅用于错误消息）。
  let name = unsafe { cstr_text(name_ptr) };
  // Safety: 末分支按 cpp 抛「未知字段」Lua 错误，该调用不返回。
  unsafe { luaL_error!(l, "unknown field {name}") }
}
