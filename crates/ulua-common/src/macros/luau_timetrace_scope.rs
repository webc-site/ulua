//! `LUAU_TIMETRACE_SCOPE(name, category)`. Reference:
//! `luau/Common/include/Luau/TimeTrace.h`.
//!
//! The TimeTrace profiling machinery (`Scope`, `createScopeData`, `ThreadContext`
//! …) lives behind C++ `#if defined(LUAU_ENABLE_TIME_TRACE)`, which is OFF in the
//! default build — there the macro is the empty `#else` form. This is that
//! default: a no-op, so downstream profiling-instrumented code compiles. The
//! machinery itself is already hand-ported in this crate behind the
//! `luau_enable_time_trace` feature（见 `records::thread_context`），尚未接线的只是
//! 宏的启用形态。b28 裁定：该形态未来的下游展开 ABI
//! （`create_scope_data`、`Scope`、`with_thread_context`、
//! `ThreadContext::event_argument`）保留 `pub`，镜像链其余机制项已降 `pub(crate)`。
//!
//! ## 启用方式（r7-timetrace 保留裁定，登记于此）
//! - 编译面：`cargo build --features ulua-common/luau_enable_time_trace`。该 feature
//!   唯一定义点在 `ulua-common/Cargo.toml`；workspace 全员、examples/fuzz、`sh/`、
//!   CI 与 Makefile 均未启用 ⇒ 默认所有构建 `thread_context::ENABLED == false`，
//!   等价 cpp 未定义 `LUAU_ENABLE_TIME_TRACE` 的形态。
//! - 运行时面：FFlag `DebugLuauTimeTracing`（[`crate::fflag::DEBUG_LUAU_TIME_TRACING]`）
//!   可达置位点：CLI `--timetrace`（analyze/repl/compile 三处）与
//!   `--fflags=DebugLuauTimeTracing[=true]` 按名置位。注意 rt 启动期
//!   `set_luau_bool_flags(true)` 的整体点亮谓词只认 `Luau*` 前缀，**不会**点亮本旗
//!   （`Debug*` 前缀被 `is_default_enabled_flag` 排除）。
//! - 当前接线状态：宏启用形态未落地（本宏恒为 `#else` 空形态），即使打开 feature
//!   也仅激活 `ThreadContext` 的登记簿记，不产生 trace 输出；完整启用还需给宏补
//!   启用臂（经 `create_scope_data` + `Scope` 展开），属未来票面。

#[macro_export]
macro_rules! LUAU_TIMETRACE_SCOPE {
  ($name:expr, $category:expr) => {};
}

/// `LUAU_TIMETRACE_ARGUMENT(name, value)`. Reference:
/// `luau/Common/include/Luau/TimeTrace.h`. No-op in the default build
/// (`LUAU_ENABLE_TIME_TRACE` off — the C++ `#else` is `do {} while (false)`); see
/// [`crate::macros::luau_timetrace_scope`] for the feature-gating note.
#[macro_export]
macro_rules! LUAU_TIMETRACE_ARGUMENT {
  ($name:expr, $value:expr) => {};
}

pub use LUAU_TIMETRACE_ARGUMENT;
pub use LUAU_TIMETRACE_SCOPE;
