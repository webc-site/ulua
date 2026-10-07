//! analysis 侧向 Lua VM 抛出类型函数运行期错误的唯一收口。
//!
//! cpp 的 `luaL_error(L, "%s", msg)` 形参里带一个 C 格式串，其内容在本端口不参与格式化
//! （真正的文本由 `format_args!` 产物完成）。VM 门面 `lua_l_error_l` 已删去该形参（见其
//! DELIBERATE DEVIATION 注记），本 crate 的 40 个运行期入口因此无需再书写任何 `c"..."`
//! 字面量。

use core::fmt::Arguments;

use ulua_vm::{functions::lua_l_error_l::lua_l_error_l, records::lua_state::LuaState};

/// cpp 尚未实现的「读写分离 indexer」错误正文，`type.setreadindexer` 与
/// `type.setwriteindexer` 两个入口共用（各自再冠以自己的方法名前缀）。
pub(crate) const SEPARATE_RW_INDEXER_MSG: &str =
  "luau does not yet support separate read/write types for indexers.";

/// 以 `args` 为消息文本，经 `luaL_error` 语义向当前 VM 调用帧抛出错误（不返回）。
///
/// 本函数是 safe fn：形参为 `&mut LuaState` 与 owned 的 [`Arguments`]，存活/独占由引用
/// 类型承载；`lua_l_error_l` 已随 wave-6d 降为引用形安全门面，直传借用、无 `unsafe`
/// 残留，调用方无需承担任何内存安全前提。
///
/// 调用序契约（正确性，非内存安全）：`l` 应处于原生函数调用的受保护执行帧内、栈上
/// 留有 `lua_l_error_l` 语义所需的空间；违约时 VM 错误路径以 panic/abort 确定性收敛，
/// 不构成 UB。本函数末尾必抛，故返回 `!`。
pub(crate) fn throw_type_error(l: &mut LuaState, args: Arguments<'_>) -> ! {
  lua_l_error_l(l, args)
}
