use alloc::{collections::BTreeMap, string::String};
use core::{
  ptr::NonNull,
  sync::atomic::{AtomicBool, AtomicU64},
};
use std::thread::JoinHandle;

use ulua_vm::records::lua_callbacks::LuaCallbacks;

/// GC 状态槽数量（cpp Profiler.cpp 的 `uint64_t gc[16]`）
pub(crate) const GC_STATE_COUNT: usize = 16;

pub(crate) struct Profiler {
  /// cpp `Profiler::callbacks`（可空）：采样线程 shim 未接线时为 `None`，
  /// null 哨兵用 Option 表达；布局与 `*mut` 逐位相同，字段分区契约不变。
  pub(crate) callbacks: Option<NonNull<LuaCallbacks>>,
  pub(crate) frequency: i32,
  pub(crate) thread: Option<JoinHandle<()>>,
  pub(crate) exit: AtomicBool,
  pub(crate) ticks: AtomicU64,
  pub(crate) samples: AtomicU64,
  pub(crate) current_ticks: u64,
  pub(crate) stack_scratch: String,
  pub(crate) data: BTreeMap<String, u64>,
  pub(crate) gc: [u64; GC_STATE_COUNT],
}
