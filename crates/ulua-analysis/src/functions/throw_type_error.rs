//! analysis 侧向 Lua VM 抛出类型函数运行期错误的唯一收口。
//!
//! cpp 的 `luaL_error(L, "%s", msg)` 形参里带一个 C 格式串；Rust 侧真正生效的格式化
//! 由 `format_args!` 产物完成，VM 门面 `lua_l_error_l` 仅为镜像 C 签名而保留该形参并
//! 忽略其内容。本 crate 不是 C ABI 实现层（见 lib.rs 总则），故把恒为 `"%s"` 的形参
//! 收敛到这里的一处 `b"..\0"` 常量，40 个运行期入口不再各自书写 `c"..."` 字面量。

use core::fmt::Arguments;

use ulua_vm::{functions::lua_l_error_l::lua_l_error_l, records::lua_state::LuaState};

/// `luaL_error` 的 C 格式串形参：NUL 结尾静态字节串，内容被 VM 忽略。
const IGNORED_FORMAT: &[u8] = b"%s\0";

/// cpp 尚未实现的「读写分离 indexer」错误正文，`type.setreadindexer` 与
/// `type.setwriteindexer` 两个入口共用（各自再冠以自己的方法名前缀）。
pub(crate) const SEPARATE_RW_INDEXER_MSG: &str =
  "luau does not yet support separate read/write types for indexers.";

/// 以 `args` 为消息文本，经 `luaL_error` 语义向当前 VM 调用帧抛出错误（不返回）。
///
/// # Safety
/// `l` 必须是当前存活、处于可抛出错误的受保护调用帧内、且栈上至少留有两个空闲槽的
/// `lua_State`；调用方须保证单线程独占该 VM 栈。本函数末尾 `lua_error` 必抛，故 `!`。
pub(crate) unsafe fn throw_type_error(l: *mut LuaState, args: Arguments<'_>) -> ! {
  // Safety: `IGNORED_FORMAT` 是静态 NUL 结尾串（`as_ptr().cast()` 指向 'static 缓冲区，
  // 调用期内有效）；`args` 是调用方现构的 `format_args!` 产物，占位符与实参在
  // `format_args!` 处即静态匹配；`l` 的有效性由本函数前置条件交给 `lua_l_error_l`。
  unsafe { lua_l_error_l(l, IGNORED_FORMAT.as_ptr().cast(), args) }
}
