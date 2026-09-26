use crate::{
  functions::bit_map1::bit_map1, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
  type_aliases::b_uint::BUint,
};

/// # Safety
///
/// `l` 必须指向本次 binary32 C 函数调用的存活 `LuaState`：所需实参按 API 索引约定位于栈上可读（越界或非数值由 check*/argerror 报错），栈顶预留结果空间。
pub(crate) unsafe fn b_swap(l: *mut LuaState) -> i32 {
  // Safety: 契约同上；BUint=u32 时 swap_bytes 与原手抄四段移位逐位恒等（同族 int_64_bswap 先例）
  unsafe { bit_map1(l, |n: BUint| n.swap_bytes()) }
}

lua_lib_fn!(pub(crate) fn b_swap, b_swap_arm);
