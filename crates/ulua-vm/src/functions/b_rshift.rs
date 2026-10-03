use crate::{
  functions::{b_shift::b_shift, lua_l_checkunsigned::lua_l_checkunsigned},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：
/// 调用方须保证本次 binary32 C 函数调用实参栈槽按索引可读、栈顶留有压入结果的 LUA_MINSTACK 余量。
pub(crate) fn b_rshift(l: &mut LuaState) -> i32 {
  // wrapping_neg avoids UB on INT_MIN (C++ negates a plain `int`); b_shift
  // treats the magnitude via unsigned_abs, so the wrapped value is handled.
  let r = lua_l_checkunsigned(l, 1);
  let i = l.check_integer(2).wrapping_neg();
  b_shift(l, r, i)
}

lua_lib_fn!(pub(crate) fn b_rshift @ref, b_rshift_arm);
