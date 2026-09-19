use crate::records::task_scheduler::{Task, TaskScheduler};

/// `void push(std::function<void()> task)` (`CLI/src/Analyze.cpp:365-373`):
///
/// ```cpp
/// {
///     std::unique_lock guard(mtx);
///     tasks.push(std::move(task));
/// }
/// cv.notify_one();
/// ```
pub fn task_scheduler_push(this: &TaskScheduler, task: Task) {
  {
    let mut guard = this.tasks.mtx.lock().unwrap();
    guard.push_back(task);
  }

  this.tasks.cv.notify_one();
}
