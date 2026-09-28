//! crate 内部共用：取栈顶函数的 `short_src`（coverage/counters dump 的公共前置步，
//! 合并两处逐字重复的 `LuaDebug{}` + `lua_getinfo("s")` + 判空取串序列）。

use alloc::string::String;
use core::mem::zeroed;

use ulua_common::functions::c_str::cstr_cow;
use ulua_vm::{
  functions::lua_getinfo::lua_getinfo,
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

/// `lua_getinfo` 选项串「取 short_src」（NUL 结尾字节串，收口点转 C 指针）。
const GETINFO_S_OPT: &[u8] = b"s\0";

/// `lua_getinfo(l, -1, "s")` 后提取 `short_src`；null 返回空串（对应 cpp 内嵌
/// `char[256]` 取不到即空的观察行为）。
///
/// # Safety
/// `l` 为有效 VM 状态且栈顶是一个可 `lua_getinfo` 的函数。
pub(crate) unsafe fn stack_function_name(l: *mut LuaState) -> String {
  // C++ `LuaDebug ar = {}`：纯 POD，全零是合法初值。
  // Safety: LuaDebug 为 POD，zeroed() 是合法全零初值。
  let mut ar: LuaDebug = unsafe { zeroed() };
  // Safety: l 与栈顶可查询的函数由 fn /// # Safety 保证；&mut ar 是本地独占借用交 lua_getinfo 填充。
  unsafe { lua_getinfo(l, -1, GETINFO_S_OPT.as_ptr().cast(), &mut ar) };
  // Safety: getinfo "s" 保证 short_src 为 NUL 结尾串或 null；后者由 cstr_cow
  // 门面译成空串（收敛判空+from_ptr 样板到单点），串缓冲存活至本帧结束。
  unsafe { cstr_cow(ar.short_src) }.into_owned()
}
