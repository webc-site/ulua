//! crate 内部共用：取栈顶函数的 `short_src`（coverage/counters dump 的公共前置步，
//! 合并两处逐字重复的 `LuaDebug{}` + `lua_getinfo("s")` + 判空取串序列）。

use alloc::string::String;

use ulua_common::functions::c_str::{cstr, cstr_cow};
use ulua_vm::{
  functions::lua_getinfo::lua_getinfo,
  records::lua_state::LuaState,
};

use crate::functions::ZERO_DEBUG;

/// `lua_getinfo` 选项串「取 short_src」（NUL 结尾字节串，经 `cstr` 收口点转 C
/// 指针，review.md §10：不散落 `.as_ptr().cast()`）。
const GETINFO_S_OPT: &[u8] = b"s\0";

/// `lua_getinfo(l, -1, "s")` 后提取 `short_src`；null 返回空串（对应 cpp 内嵌
/// `char[256]` 取不到即空的观察行为）。
///
/// `l` 为有效 VM 状态且栈顶是一个可 `lua_getinfo` 的函数（调用点守卫）。
// DELIBERATE DEVIATION（review.md §9.3）：`lua_getinfo` 出参 `LuaDebug`（含 NUL 数组）
// 的 c-API 边界；编译期零初值（见 functions::ZERO_DEBUG），short_src 判空/截断经
// cstr_cow 门面收敛为 `String`。
pub(crate) fn stack_function_name(l: &mut LuaState) -> String {
  // C++ `LuaDebug ar = {}`：编译期零初值（见 functions::ZERO_DEBUG）
  let mut ar = ZERO_DEBUG;
  // Safety: `lua_getinfo` 为 unsafe 导出；`&mut ar` 是本地独占借用交 lua_getinfo 填充，
  // 前提（栈顶可查询的函数）由调用点守卫成立；what 为 NUL 结尾静态字节串（`cstr`
  // 门面）。
  unsafe { lua_getinfo(l, -1, cstr(GETINFO_S_OPT), &mut ar) };
  // Safety: `cstr_cow` 为 unsafe fn；getinfo "s" 保证 short_src 为 NUL 结尾串或 null，
  // 后者由门面译成空串（收敛判空+from_ptr 样板到单点），串缓冲存活至本帧结束。
  unsafe { cstr_cow(ar.short_src) }.into_owned()
}
