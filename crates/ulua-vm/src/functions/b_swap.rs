use crate::{
  functions::bit_map1::bit_map1, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
  type_aliases::b_uint::BUint,
};

/// byteswap 核心：套 `bit_map1` 翻字节。
///
/// 调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调——1 号实参按 API 索引约定
/// 位于栈上可读（越界或非数值由 check*/argerror 报错回退），栈顶预留结果空间。
/// BUint=u32 时 `swap_bytes` 与原手抄四段移位逐位恒等（同族 int_64_bswap 先例）。
pub(crate) fn b_swap(l: &mut LuaState) -> i32 {
  bit_map1(l, |n: BUint| n.swap_bytes())
}

lua_lib_fn!(pub(crate) fn b_swap @ref, b_swap_arm);
