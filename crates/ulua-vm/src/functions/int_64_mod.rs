use crate::{
  functions::{check_div_args_64::check_nonzero_divisor, int_64_shared::int64_divop},
  macros::lua_lib_arm::lua_lib_arm,
};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `LuaState` 且处于受保护帧：`int64_divop` 经 `luaL_checkinteger64` 读索引 1/2
  /// （非整数抛错回退）；除数为 0 时经 `check_nonzero_divisor` 抛错回退；`lua_pushinteger_64`
  /// 需 `(*l).top` 后 ≥1 空槽；可触发 GC。cpp VM/src/lintlib.cpp:153。
  pub fn int64_mod(l) {
    unsafe {
      int64_divop(l, |l, _a, b| check_nonzero_divisor(l, b as u64), |a, b| {
        // cpp：INT64_MIN % -1 溢出特判为 0；余数非 0 且被除数/除数异号时
        // `remainder += b`（结果随除数符号，floor mod）
        if a == i64::MIN && b == -1 {
          return 0;
        }
        let r = a % b;
        if r != 0 && ((a < 0) != (b < 0)) {
          r + b
        } else {
          r
        }
      })
    }
  }
}
