use crate::{
  functions::{
    b_shift::b_shift, lua_l_checkunsigned::lua_l_checkunsigned, lua_pushunsigned::lua_pushunsigned,
  },
  macros::{lua_lib_fn::lua_lib_fn, nbits::NBITS, trim::trim},
  records::lua_state::LuaState,
  type_aliases::b_uint::BUint,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：
/// 调用方须保证本次 binary32 C 函数调用所需实参按 API 索引约定位于栈上可读
/// （越界或非数值由 check*/argerror 报错路径抛出）、栈顶预留压入结果的 LUA_MINSTACK 余量。
pub(crate) fn b_arshift(l: &mut LuaState) -> i32 {
  let mut r: BUint = lua_l_checkunsigned(l, 1);
  let i: i32 = l.check_integer(2);

  // C: `if (i < 0 || !(r & ((BUint)1 << (NBITS - 1))))` — logical NOT: sign bit clear.
  if i < 0 || (r & ((1 as BUint) << (NBITS as u32 - 1))) == 0 {
    // wrapping_neg avoids UB on INT_MIN (C++ negates a plain `int`).
    return b_shift(l, r, i.wrapping_neg());
  }

  // arithmetic shift for 'negative' number
  if i >= NBITS {
    r = !0 as BUint;
  } else {
    r = trim((r >> i as u32) | !(!(0 as BUint) >> i as u32)); // add signal bit
  }

  lua_pushunsigned(l, r);
  1
}

lua_lib_fn!(pub(crate) fn b_arshift @ref, b_arshift_arm);
