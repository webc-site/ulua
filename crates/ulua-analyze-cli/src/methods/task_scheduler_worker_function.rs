/// `void workerFunction()` (`CLI/src/Analyze.cpp:381-385`):
///
/// ```cpp
/// while (std::function<void()> task = pop())
///     task();
/// ```
/// An empty `std::function` (modeled as `None`) is falsy and terminates the loop.
use crate::methods::task_scheduler_pop::task_scheduler_pop;
use crate::records::task_scheduler::TaskQueue;
pub fn task_scheduler_worker_function(tasks: &TaskQueue) {
  while let Some(task) = task_scheduler_pop(tasks) {
    task();
  }
}
