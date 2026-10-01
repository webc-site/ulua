use core::{ffi::c_int, str::from_utf8};

use ulua_vm::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

use crate::common::functions::{
  k_int_64_tag::K_INT_64_TAG,
  safe_api::{state_mut, touserdatatagged},
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn int_64_newindex(l: *mut LuaState) -> c_int {
  // 按 tag 取参数 1 的 userdata 数据指针，tag 不匹配时返回 null（下一行即转类型错误）。
  let p = touserdatatagged(l, 1, K_INT_64_TAG);
  if p.is_null() {
    // 按 cpp 以「不是 int64」抛类型错误，不返回。
    state_mut(l).type_error(1, "int64");
  }

  let name = state_mut(l).check_bytes(2);

  if name == b"value" {
    // 参数 3 为数字时返回其值（否则抛 Lua 错误）。
    let value = state_mut(l).check_number(3);
    // `p` 已排除 null，即 tag 为 K_INT_64_TAG 的 userdata 数据区，按 cpp 以 i64 写入。
    // Safety: 数据区指针由上面 tag 校验 + 判空兜底，仅本行写一次。
    unsafe { *(p as *mut i64) = value as i64 };
    return 0;
  }

  let name_str = from_utf8(name).unwrap_or("");
  // 末分支按 cpp 抛「未知字段」Lua 错误，该调用不返回。
  // Safety: `l` 存活；`luaL_error` 以 long-jump 终止本回调。
  unsafe { luaL_error!(l, "unknown field {name_str}") }
}
