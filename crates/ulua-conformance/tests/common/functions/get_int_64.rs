use ulua_vm::{
  functions::{lua_isnumber::lua_isnumber, lua_touserdatatagged::lua_touserdatatagged},
  macros::{lua_l_typeerror::luaL_typeerror, lua_tointeger::lua_tointeger},
  records::lua_state::LuaState,
};

use crate::common::functions::k_int_64_tag::K_INT_64_TAG;

pub(crate) fn get_int_64(l: *mut LuaState, idx: i32) -> i64 {
  // Safety: `l` 为本用例存活的 LuaState；按 tag 取参数 `idx` 的 userdata 数据指针，
  // tag 不匹配时返回 null。
  let p = unsafe { lua_touserdatatagged(l, idx, K_INT_64_TAG) };
  if !p.is_null() {
    // Safety: 上面已排除 null，`p` 即 tag 为 K_INT_64_TAG 的 userdata 数据区，可读 i64。
    return unsafe { *(p as *const i64) };
  }

  // Safety: `l` 存活；非 userdata 时按数字读取（`lua_tointeger!` 不可转换时返回 0）。
  if unsafe { lua_isnumber(l, idx) } != 0 {
    return unsafe { lua_tointeger!(l, idx) as i64 };
  }

  // Safety: 两类都不是时按 cpp 抛「不是 int64」的类型错误（`l` 存活），该调用不返回。
  unsafe { luaL_typeerror!(l, 1, "int64") }
}
