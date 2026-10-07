//! crate 内部共用：取栈顶函数的 `short_src`（coverage/counters dump 的公共前置步，
//! 合并两处逐字重复的 `LuaDebug` + `lua_getinfo("s")` + 取串序列）。

use alloc::string::String;

use ulua_vm::{
  functions::lua_getinfo::lua_getinfo,
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

/// `lua_getinfo` 选项模板「取 short_src」（§10：模板即字节切片，直接传参）。
const GETINFO_S_OPT: &[u8] = b"s";

/// `lua_getinfo(l, -1, "s")` 后提取 `short_src`；未填写返回空串（对应 cpp 内嵌
/// `char[256]` 取不到即空的观察行为）。
///
/// `l` 为有效 VM 状态且栈顶是一个可 `lua_getinfo` 的函数（调用点守卫）。
// DELIBERATE DEVIATION（review.md §9.3）：`lua_getinfo` 出参 `LuaDebug` 的 c-API
// 边界；`short_src` 是 VM 填写时拷成的 owned `Vec<u8>`，此处 lossy 转 `String`。
pub(crate) fn stack_function_name(l: &mut LuaState) -> String {
  let mut ar = LuaDebug::default();
  // Safety: `lua_getinfo` 为 unsafe 导出；`&mut ar` 是本地独占借用交 lua_getinfo 填充，
  // 前提（栈顶可查询的函数）由调用点守卫成立；what 为静态选项模板切片。
  unsafe { lua_getinfo(l, -1, GETINFO_S_OPT, &mut ar) };
  String::from_utf8_lossy(ar.short_src.as_deref().unwrap_or(b"")).into_owned()
}
