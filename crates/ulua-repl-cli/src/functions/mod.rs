//! 实现层模块清单。`pub` 项一一对应 cpp `Repl.h` / `ReplRequirer.h` 的导出，
//! `pub(crate)` 项对应同名 cpp 文件里的 `static`。

use core::ptr::{null, null_mut};

use ulua_vm::{macros::lua_idsize::LUA_IDSIZE, records::lua_debug::LuaDebug};

/// cpp `LuaDebug ar;` / `LuaDebug ar = {}` 的零初值（profiler 采样与
/// `stack_function_name` 共用）：整块在编译期成形，免 `mem::zeroed()` 的
/// unsafe 与「POD 全零合法」的口头论证（与 ulua-web `run_code::ZERO_DEBUG`
/// 同款形态）。
///
/// FFI: c-API 出参槽——`LuaDebug` 是 VM 的 C ABI 镜像结构（`#[repr(C)]`，出处
/// `VM/include/lua.h:488-502`），指针字段的本体即 C 侧 `char*`/`void*`；结构
/// 声明与 `luau_callhook` 写端均在 ulua-vm（本 crate 外），单端改「可空借用
/// 指针」形态不成立。此处 `null()`/`null_mut()` 仅作 const POD 初值，
/// 从不被本侧解引用。
pub(crate) const ZERO_DEBUG: LuaDebug = LuaDebug {
  name: null(),
  what: null(),
  source: null(),
  short_src: null(),
  linedefined: 0,
  currentline: 0,
  protoid: 0,
  bytecodeid: 0,
  nupvals: 0,
  nparams: 0,
  isvararg: 0,
  userdata: null_mut(),
  ssbuf: [0; LUA_IDSIZE as usize],
};

/// C-ABI 回调外壳单源模板。`lua_getcounters` / `lua_getcoverage` 只接受
/// `Option<unsafe extern "C-unwind" fn(..*mut c_void..)>`（见 ulua-vm type_aliases），
/// 而真正的计数/覆盖逻辑写在同名**安全** Rust fn 里（`coverage_callback` 还带
/// `Write` 泛型，无法直接充当 C ABI）；VM 裸指针的判空与转借（cstr_cow/c_slice/
/// context 重建）全部收口在外壳臂内，核心 fn 只见 Rust 类型。本宏集中表述
/// 「FFI 外壳」这一共同形式：
/// 属性文档 + `$vis unsafe extern "C-unwind" fn 名(形参…) { 体 }` 骨架（体省略
/// `-> ()`，避免 `unused_unit`）。形参表与 `$body` 由臂给出，`$body` 内自写
/// `unsafe {}` 转调具体核心，使「调用了哪个不安全函数」在调用点可见——宏定义体内
/// 不含任何 metavariable 落进 unsafe 块，edition 2024 下不触发
/// `clippy::macro_metavars_in_unsafe`（与 ulua-vm `lua_lib_arm` 同纪律）。
macro_rules! c_abi_cb {
  ($(#[$meta:meta])* $vis:vis fn $name:ident($($arg:ident : $ty:ty),* $(,)?) $body:block) => {
    $(#[$meta])*
    $vis unsafe extern "C-unwind" fn $name($($arg : $ty),*) $body
  };
}

// —— cpp `Repl.h` / `ReplRequirer.h` 导出的入口（供 ulua-cli-test 夹具复用）——
pub mod create_cli_require_context;
pub mod get_completions;
pub mod repl_main;
pub mod run_code;
pub mod setup_state;

// —— 以下为 Repl.cpp / ReplRequirer.cpp 的 crate 内部实现（cpp `static`）——
pub(crate) mod compile_source;
pub(crate) mod complete_indexer;
pub(crate) mod complete_partial_matches;
pub(crate) mod complete_repl;
pub(crate) mod copts;
pub(crate) mod counters_active;
pub(crate) mod counters_dump;
pub(crate) mod counters_function_callback;
pub(crate) mod counters_init;
pub(crate) mod counters_track;
pub(crate) mod counters_value_callback;
pub(crate) mod coverage_active;
pub(crate) mod coverage_callback;
pub(crate) mod coverage_dump;
pub(crate) mod coverage_init;
pub(crate) mod coverage_track;
pub(crate) mod create_dump_writer;
pub(crate) mod get_file_path;
pub(crate) mod ic_get_completions;
pub(crate) mod load;
pub(crate) mod load_history;
pub(crate) mod lua_loadstring;
pub(crate) mod main;
pub(crate) mod profiler_dump;
pub(crate) mod profiler_loop;
pub(crate) mod profiler_start;
pub(crate) mod profiler_stop;
pub(crate) mod profiler_trigger;
pub(crate) mod run_file;
pub(crate) mod run_repl;
pub(crate) mod run_repl_impl;
pub(crate) mod sigint_callback;
pub(crate) mod sigint_handler_repl;
// 收口门面：Ctrl-C 处理的注册/撤回（libc FFI 与 null 协议值只留这里）
pub(crate) mod sigint_setup;
pub(crate) mod stack_function_name;
// `*mut LuaState` 入口解引用门面（各 VM 边界入口共用，契约见模块头）
pub(crate) mod state_ref;
