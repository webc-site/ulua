use alloc::{boxed::Box, collections::VecDeque, sync::Arc, vec::Vec};
/// A `std::function<void()>` task. `None` models an empty `std::function` — the
/// falsy sentinel the C++ pushes (`push({})`) to terminate a worker's loop.
use core::cmp::max;
use core::fmt::{Debug, Formatter, Result};
use std::{
  sync::{Condvar, Mutex},
  thread::{JoinHandle, available_parallelism},
};

use crate::methods::task_scheduler_task_scheduler_analyze_alt_b::task_scheduler_destructor;
pub type Task = Option<Box<dyn FnOnce() + Send + 'static>>;

/// Port of `struct TaskScheduler` (`CLI/src/Analyze.cpp:323-392`).
///
/// A thread pool scheduler for parallel task execution, mirroring the C++ design:
/// `std::mutex` + `std::condition_variable` guarding a `std::queue<std::function<void()>>`,
/// with one `std::thread` per worker. Native-only; not portable to wasm32-unknown-unknown.
#[derive(Debug)]
pub struct TaskScheduler {
  pub(crate) thread_count: u32,
  pub(crate) workers: Vec<JoinHandle<()>>,
  pub(crate) tasks: Arc<TaskQueue>,
}

/// The shared `mtx` / `cv` / `tasks` triple from the C++ `TaskScheduler`.
pub struct TaskQueue {
  pub(crate) mtx: Mutex<VecDeque<Task>>,
  pub(crate) cv: Condvar,
}

impl Debug for TaskQueue {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("TaskQueue").finish_non_exhaustive()
  }
}

impl TaskQueue {
  pub(crate) fn new() -> Self {
    Self {
      mtx: Mutex::new(VecDeque::new()),
      cv: Condvar::new(),
    }
  }
}

impl TaskScheduler {
  /// `static unsigned getThreadCount()` (`CLI/src/Analyze.cpp:375-378`):
  /// `return std::max(std::thread::hardware_concurrency(), 1u);`
  pub fn get_thread_count() -> u32 {
    max(
      available_parallelism().map(|n| n.get() as u32).unwrap_or(1),
      1,
    )
  }
}

impl Drop for TaskScheduler {
  /// `~TaskScheduler()` (`CLI/src/Analyze.cpp:339-346`):
  /// pushes one empty task per worker, then joins them.
  fn drop(&mut self) {
    task_scheduler_destructor(self);
  }
}
