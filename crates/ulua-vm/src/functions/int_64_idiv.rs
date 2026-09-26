use crate::{
  functions::{check_div_args_64::check_div_args_64, int_64_shared::int64_divop},
  macros::lua_lib_arm::lua_lib_arm,
};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `LuaState` 且处于受保护帧：`int64_divop` 经 `luaL_checkinteger64` 读索引 1/2
  /// （非整数抛错回退）；除数为 0 或 `i64::MIN / -1` 时经 `check_div_args_64` 抛错回退；
  /// `lua_pushinteger_64` 需 `(*l).top` 后 ≥1 空槽；可触发 GC。cpp VM/src/lintlib.cpp:115。
  pub fn int64_idiv(l) {
    unsafe {
      int64_divop(l, |l, a, b| check_div_args_64(l, a, b), |a, b| {
        let quotient = a / b;
        // cpp：商为负且仍有余数时再减 1（floor div）
        if quotient < 0 && a % b != 0 {
          quotient - 1
        } else {
          quotient
        }
      })
    }
  }
}
