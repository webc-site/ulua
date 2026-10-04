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
/// # Safety
/// `l` 必须是当前存活、处于可抛出错误的受保护调用帧内、且栈上至少留有两个空闲槽的
/// `LuaState`；调用方须保证单线程独占该 VM 栈。本函数末尾 `lua_error` 必抛，故 `!`。
pub(crate) unsafe fn throw_type_error(l: &mut LuaState, args: Arguments<'_>) -> ! {
  // Safety: 前置条件即本 fn 契约（`l` 存活受保护、栈留 2 空槽），原样透传给
  // `lua_l_error_l`；`args` 由调用点 `format_args!` 现场构造，占位符与实参静态匹配。
  unsafe { lua_l_error_l(l.as_mut_ptr(), args) }
}
