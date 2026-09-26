//! crate 内部共用的小工具函数与常量。

use std::{string::String, sync::Once};

use ulua_common::set_luau_bool_flags;
use ulua_vm::macros::lua_memerrmsg::LUA_MEMERRMSG_STR;

/// VM 全局 `print` 的 C 名称（wasm 捕获版的注册与解注册共用；`run_code` 的
/// 结果打印已下沉 `ulua_vm::functions::run_loaded_chunk`，其内同名常量随迁）。
#[cfg(feature = "wasm")]
pub(crate) const PRINT_NAME: &[u8] = b"print\0";

/// `luaL_newstate` / `lua_newthread` 分配失败（内存耗尽）时回报宿主的错误文本；
/// VM 状态为 null 时不得继续解引用。直接复用 VM 的内存错误常量，避免同一字面量
/// 在多处漂移。
pub(crate) const NOT_ENOUGH_MEMORY: &str = LUA_MEMERRMSG_STR;

/// 进程内「置默认 FastFlag」一次性门。
///
/// 上游 `setLuauFlagsDefault()` 只在进程入口调用一次
/// （`CLI/src/ReplEntry.cpp:7`、`CLI/src/Analyze.cpp:399`）；`CLI/src/Web.cpp:186-189`
/// 虽然写在 `executeScript` 内，但 C++ 宿主隐含单线程。Rust 侧这些入口可被宿主
/// 从任意线程并发调用，故收敛为一次性。
static INIT_LUAU_FLAGS: Once = Once::new();

/// 首次调用时把 `Luau*` bool FastFlag 置为默认值，之后视为只读。
///
/// FastFlag 是进程级 `UnsafeCell` 全局，ulua-common 的契约是「仅在线程启动前
/// 调用」（见 `records/f_value.rs::set_luau_bool_flags`）：per-call 裸写会与另一
/// 线程的同名写入或与并发只读 `get()` 构成数据竞争（Rust 语义下即 UB），也会
/// 静默覆盖宿主先前配置的 flag。本函数的并发安全性由 [`Once`] 提供。
pub(crate) fn init_default_flags() {
  INIT_LUAU_FLAGS.call_once(|| set_luau_bool_flags(true));
}

/// cpp 两个入口共用的收尾：空结果即「无诊断 / 执行成功」，回 `None`
/// （`Web.cpp:181`、`Web.cpp:207` 的 `empty() ? nullptr : c_str()`）。
///
/// 两个入口的返回装配完全同构，故收成一个函数而不是各写一遍三元式。
pub(crate) fn nullable(text: String) -> Option<String> {
  (!text.is_empty()).then_some(text)
}
