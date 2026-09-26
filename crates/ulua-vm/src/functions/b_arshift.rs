use crate::{
  functions::{
    b_shift::b_shift, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checkunsigned::lua_l_checkunsigned, lua_pushunsigned::lua_pushunsigned,
  },
  macros::{nbits::NBITS, trim::trim},
  records::lua_state::LuaState,
  type_aliases::b_uint::BUint,
};

/// # Safety
///
/// `l` 必须指向本次 binary32 C 函数调用的存活 `LuaState`：所需实参按 API 索引约定位于栈上可读（越界或非数值由 check*/argerror 报错），栈顶预留结果空间。
pub(crate) unsafe extern "C-unwind" fn b_arshift(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `l` 指向本次 binary32 C 函数调用的存活 LuaState，实参栈槽按索引可读、栈顶留有压入结果的 LUA_MINSTACK 余量
  unsafe {
    let mut r: BUint = lua_l_checkunsigned(l, 1);
    let i: i32 = lua_l_checkinteger(l, 2);

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
}
