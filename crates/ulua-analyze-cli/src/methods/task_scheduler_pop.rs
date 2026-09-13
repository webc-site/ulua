use crate::records::task_scheduler::{Task, TaskQueue};

/// `std::function<void()> pop()` (`CLI/src/Analyze.cpp:348-363`):
///
/// ```cpp
/// std::unique_lock guard(mtx);
/// cv.wait(guard, [this] { return !tasks.empty(); });
/// std::function<void()> task = tasks.front();
/// tasks.pop();
/// return task;
/// ```
pub fn task_scheduler_pop(tasks: &TaskQueue) -> Task {
  let mut guard = tasks.mtx.lock().unwrap();

  // cv.wait(guard, [] { return !tasks.empty(); });
  guard = tasks.cv.wait_while(guard, |q| q.is_empty()).unwrap();

  guard.pop_front().flatten()
}
