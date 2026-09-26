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

#[macro_export]
macro_rules! LUAU_TIMETRACE_SCOPE {
  ($name:expr, $category:expr) => {};
}

pub use LUAU_TIMETRACE_SCOPE;
