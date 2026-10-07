//! cpp `createCliRequireContext`（`CLI/src/Repl.cpp`）。
//!
//! 原实现把 `ReplRequirer` placement 构造进 `lua_newuserdatadtor` userdata 并以
//! 地址为键登记 registry，供 C 回调按裸指针重建引用；trait 化后宿主所有权直接
//! 交给 `ulua-require`（`push_closure` 把它装箱为闭包 userdata，GC 终结），
//! 此处只剩纯构造。返回不透明 `impl RequireHost` 以免外泄 crate 内部类型。

use ulua_require::records::navigation_context::RequireHost;

use crate::{
  functions::{
    copts::copts, counters_active::counters_active, counters_track::counters_track,
    coverage_active::coverage_active, coverage_track::coverage_track,
    repl_main::repl_codegen_enabled,
  },
  records::repl_requirer::ReplRequirer,
};

/// 构造 REPL 的 require 宿主（对应 cpp 六个探针入参的 `ReplRequirer`）。
pub fn create_cli_require_context() -> impl RequireHost + 'static {
  // 对应 cpp `new (ctx) ReplRequirer{ copts, coverageActive,
  // []{ return codegen; }, coverageTrack, countersActive, countersTrack }`，
  // 六个探针均为 Rust 内部调用的普通函数指针。
  ReplRequirer::new(
    copts,
    coverage_active,
    repl_codegen_enabled,
    coverage_track,
    counters_active,
    counters_track,
  )
}
