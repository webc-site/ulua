use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::{
  k_int_64_tag::K_INT_64_TAG,
  safe_api::{isnumber, state_mut, touserdatatagged},
};

pub(crate) fn get_int_64(l: *mut LuaState, idx: i32) -> i64 {
  // 按 tag 取参数 `idx` 的 userdata 数据指针，tag 不匹配时返回 null。
  let p = touserdatatagged(l, idx, K_INT_64_TAG);
  if !p.is_null() {
    // `p` 即 tag 为 K_INT_64_TAG 的 userdata 数据区，可读 i64。
    // Safety: 上面已排除 null，仅本行读一次。
    return unsafe { *(p as *const i64) };
  }

  // 非 userdata 时按数字读取（`lua_tointeger!` 不可转换时返回 0）。
  if isnumber(l, idx) != 0 {
    return state_mut(l).to_integer(idx).unwrap_or(0) as i64;
  }

  // 两类都不是时按 cpp 抛「不是 int64」的类型错误，该调用不返回。
  state_mut(l).type_error(1, "int64")
}
