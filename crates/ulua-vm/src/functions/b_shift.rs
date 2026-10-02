use crate::{
  functions::lua_pushunsigned::lua_pushunsigned,
  macros::{nbits::NBITS, trim::trim},
  records::lua_state::LuaState,
  type_aliases::b_uint::BUint,
};

/// # Safety
///
/// `l` 必须指向本次 binary32 C 函数调用的存活 `LuaState`：所需实参按 API 索引约定位于栈上可读（越界或非数值由 check*/argerror 报错），栈顶预留结果空间。
pub(crate) unsafe fn b_shift(l: *mut LuaState, mut r: BUint, i: i32) -> i32 {
  // Mirrors VM/src/lbitlib.cpp:b_shift
  if i < 0 {
    // Magnitude of the (right) shift. `i.unsigned_abs()` is defined for
    // `i == INT_MIN` (the C++ `i = -i` is UB there), and using it as the
    // bound also avoids a shift-by->=32 (itself UB) — |i| >= NBITS yields 0.
    let amount = i.unsigned_abs();
    r = trim(r);
    if amount >= NBITS as u32 {
      r = 0;
    } else {
      r >>= amount;
    }
  } else {
    if i >= NBITS {
      r = 0;
    } else {
      r <<= i as u32;
    }
    r = trim(r);
  }

  // SAFETY: 契约保证 `l` 为存活调用帧；本帧把裸指针重建为独占引用后即结束借用窗口
  // （`lua_pushunsigned` 自身签名安全，仅要求 `&mut LuaState`）
  unsafe {
    lua_pushunsigned(&mut *l, r);
  }
  1
}
