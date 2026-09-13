use alloc::{sync::Arc, vec::Vec};
use std::thread::spawn;

use crate::{
  methods::task_scheduler_worker_function::task_scheduler_worker_function,
  records::task_scheduler::{TaskQueue, TaskScheduler},
};
impl TaskScheduler {
  /// `TaskScheduler(unsigned threadCount)` (`CLI/src/Analyze.cpp:325-337`):
  /// spawns `threadCount` worker threads, each running `workerFunction()`.
  pub fn task_scheduler_task_scheduler(thread_count: u32) -> Self {
    let tasks = Arc::new(TaskQueue::new());
    let mut workers = Vec::with_capacity(thread_count as usize);

    for _ in 0..thread_count {
      let tasks = Arc::clone(&tasks);
      let handle = spawn(move || {
        task_scheduler_worker_function(&tasks);
      });
      workers.push(handle);
    }

    TaskScheduler {
      thread_count,
      workers,
      tasks,
    }
  }
}
