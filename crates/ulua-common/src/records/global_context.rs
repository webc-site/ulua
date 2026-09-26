//! Source: `Common/src/TimeTrace.cpp:92-110` (hand-ported)
//!
//! C++ `struct GlobalContext` holds a `std::mutex` guarding the mutable trace
//! state (`threads`, `nextThreadId`, `tokens`, `traceFile`). The Rust port keeps
//! that state behind a `parking_lot::Mutex`（无投毒语义，与 `std::mutex` 一致）
//! so the singleton is safely shareable through `Arc`, exactly as the C++
//! `std::scoped_lock(context.mutex)` does.

use alloc::vec::Vec;
use std::fs::File;

use parking_lot::{Mutex, MutexGuard};

use crate::records::token::Token;

#[derive(Debug, Default)]
pub(crate) struct GlobalContextState {
  /// cpp `std::vector<ThreadContext*> threads`：活跃线程上下文的登记表。
  ///
  /// 上游存裸指针只为 `createThread`/`releaseThread` 的配对增删（`flushEvents`
  /// 从不遍历它），指针本身不承载信息。Rust 侧改存 `threadId`——同样唯一、同样可
  /// 比较，因此不再需要 `unsafe impl Send`，也不会残留悬垂指针。
  pub(crate) threads: Vec<u32>,
  pub(crate) next_thread_id: u32,
  pub(crate) tokens: Vec<Token>,
  pub(crate) trace_file: Option<File>,
}

/// cpp `GlobalContext`（`Common/src/TimeTrace.cpp:92-110`）镜像工件，sync-cpp 维护；
/// 出现面已全收进 `pub(crate)` 函数签名，随链降 `pub(crate)`。
#[derive(Debug)]
pub(crate) struct GlobalContext {
  pub(crate) state: Mutex<GlobalContextState>,
}

impl GlobalContext {
  /// C++ `GlobalContext() = default;`（`Common/src/TimeTrace.cpp:107` 私有默认构造，
  /// 只由 `getGlobalContext` 使用）；cpp `~GlobalContext() { if (traceFile)
  /// fclose(traceFile); }` 由 `File` 的 `Drop` 承担，无需移植。
  pub(crate) fn new() -> Self {
    GlobalContext {
      state: Mutex::new(GlobalContextState::default()),
    }
  }

  /// 加锁获取可变 trace 状态。parking_lot 互斥量无投毒语义，持锁线程 panic 后
  /// 后继获取者照常工作，与 C++ `std::scoped_lock` 等价。
  pub(crate) fn lock_state(&self) -> MutexGuard<'_, GlobalContextState> {
    self.state.lock()
  }
}
