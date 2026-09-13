/// `~TaskScheduler()` (`CLI/src/Analyze.cpp:339-346`):
///
/// ```cpp
/// for (unsigned i = 0; i < threadCount; i++)
///     push({});
/// for (std::thread& worker : workers)
///     worker.join();
/// ```
use crate::methods::task_scheduler_push::task_scheduler_push;
use crate::records::task_scheduler::TaskScheduler;
pub fn task_scheduler_destructor(this: &mut TaskScheduler) {
  for _ in 0..this.thread_count {
    // push({}) — an empty std::function terminates a worker's loop.
    task_scheduler_push(this, None);
  }

  for worker in this.workers.drain(..) {
    let _ = worker.join();
  }
}
