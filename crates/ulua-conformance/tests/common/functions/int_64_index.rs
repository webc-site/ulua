use core::{ffi::c_int, str::from_utf8};

use ulua_vm::{
  functions::lua_touserdatatagged::lua_touserdatatagged, macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
};

use crate::common::functions::k_int_64_tag::K_INT_64_TAG;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn int_64_index(l: *mut LuaState) -> c_int {
  // Safety: `l` 为本用例存活的 LuaState；按 tag 取参数 1 的 userdata 数据指针，
  // tag 不匹配时返回 null（下一行即转类型错误）。
  let p = unsafe { lua_touserdatatagged(l, 1, K_INT_64_TAG) };
  if p.is_null() {
    // Safety: 按 cpp 以「不是 int64」抛类型错误，不返回。
    unsafe { (*l).type_error(1, "int64") };
  }

  let name = unsafe { (*l).check_bytes(2) };

  if name == b"value" {
    // Safety: 上面已排除 null，`p` 即 tag 为 K_INT_64_TAG 的 userdata 数据区，可读 i64；
    // `l` 存活，压入其 f64 视图。
    unsafe { (*l).push_number(*(p as *const i64) as f64) };
    return 1;
  }

  let name_str = from_utf8(name).unwrap_or("");
  // Safety: 末分支按 cpp 抛「未知字段」Lua 错误，该调用不返回。
  unsafe { luaL_error!(l, "unknown field {name_str}") }
}
