//! Source: `Common/src/TimeTrace.cpp:92-110` (hand-ported)
//!
//! C++ `struct GlobalContext` holds a `std::mutex` guarding the mutable trace
//! state (`threads`, `nextThreadId`, `tokens`, `traceFile`). The Rust port keeps
//! that state behind a `std::sync::Mutex` so the singleton is safely shareable
//! through `Arc`, exactly as the C++ `std::scoped_lock(context.mutex)` does.

use alloc::vec::Vec;
use std::{
  fs::File,
  sync::{Mutex, MutexGuard},
};

use crate::records::token::Token;

#[derive(Debug, Default)]
pub(crate) struct GlobalContextState {
  /// 已注册线程的 `thread_id` 列表（C++ `std::vector<ThreadContext*>`）。
  /// 偏差（行为等价）：Rust 的 `ThreadContext` 按值移动，`this` 指针在
  /// 构造与析构之间不稳定，故以唯一的 `thread_id` 作注册身份；该列表
  /// 只做注册/注销配对，从不解引用。
  pub(crate) threads: Vec<u32>,
  pub(crate) next_thread_id: u32,
  pub(crate) tokens: Vec<Token>,
  pub(crate) trace_file: Option<File>,
}

#[derive(Debug)]
pub struct GlobalContext {
  pub(crate) state: Mutex<GlobalContextState>,
}

impl GlobalContext {
  /// 加锁获取可变 trace 状态。锁中毒（持锁线程 panic）时沿用守卫内的数据
  /// 继续运行，与 C++ `std::scoped_lock` 的语义等价且不引入 panic。
  pub(crate) fn lock_state(&self) -> MutexGuard<'_, GlobalContextState> {
    self
      .state
      .lock()
      .unwrap_or_else(|poisoned| poisoned.into_inner())
  }
}
