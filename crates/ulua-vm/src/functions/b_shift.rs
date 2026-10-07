use crate::{
  functions::lua_pushunsigned::lua_pushunsigned,
  macros::{nbits::NBITS, trim::trim},
  records::lua_state::LuaState,
  type_aliases::b_uint::BUint,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：
/// 调用方须保证本次 binary32 C 函数调用所需实参按 API 索引约定位于栈上可读
/// （越界或非数值由 check*/argerror 报错路径抛出）、栈顶预留结果空间。
pub(crate) fn b_shift(l: &mut LuaState, mut r: BUint, i: i32) -> i32 {
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

  // `lua_pushunsigned` 签名安全、只收 `&mut LuaState`：引用形下本函数体内零裸指针触点
  lua_pushunsigned(l, r);
  1
}
