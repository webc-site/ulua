use core::cmp;
/// `static unsigned getThreadCount()` (`CLI/src/Analyze.cpp:375-378`):
/// `return std::max(std::thread::hardware_concurrency(), 1u);`
use std::thread::available_parallelism;
pub fn task_scheduler_get_thread_count() -> u32 {
  cmp::max(available_parallelism().map(|n| n.get()).unwrap_or(1), 1) as u32
}
