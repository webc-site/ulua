use crate::{
  functions::b_rot::b_rot, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：
/// 调用方须保证本次 binary32 C 函数调用实参栈槽按索引可读、栈顶留有压入结果的 LUA_MINSTACK 余量。
pub(crate) fn b_rrot(l: &mut LuaState) -> i32 {
  // wrapping_neg avoids UB on INT_MIN (C++ negates a plain `int`); b_rot masks
  // the count with `& (NBITS-1)`, so the wrapped value rotates correctly.
  let i = l.check_integer(2).wrapping_neg();
  b_rot(l, i)
}

lua_lib_fn!(pub(crate) fn b_rrot @ref, b_rrot_arm);
