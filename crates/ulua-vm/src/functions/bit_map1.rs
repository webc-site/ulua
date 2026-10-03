use crate::{
  functions::{lua_l_checkunsigned::lua_l_checkunsigned, lua_pushunsigned::lua_pushunsigned},
  records::lua_state::LuaState,
  type_aliases::b_uint::BUint,
};

/// 单目 bit32 库函数骨架：取 1 号无符号实参，套 `f` 后压回，返回 1。
///
/// 调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调——1 号实参按 API 索引约定
/// 位于栈上可读（越界或非数值由 check*/argerror 报错回退），栈顶预留结果空间；
/// `f` 为纯位计算闭包。
#[inline]
pub(crate) fn bit_map1(l: &mut LuaState, f: impl Fn(BUint) -> BUint) -> i32 {
  let v = lua_l_checkunsigned(l, 1) as BUint;
  lua_pushunsigned(l, f(v));
  1
}
