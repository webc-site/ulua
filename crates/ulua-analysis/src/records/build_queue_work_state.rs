use alloc::{boxed::Box, vec::Vec};
use std::sync::{Condvar, Mutex};

use crate::records::build_queue_item::BuildQueueItem;

/// C++: `using Task = std::function<void()>;` — a single unit of queued work.
pub type Task = Box<dyn FnOnce() + Send>;

pub struct BuildQueueWorkState {
  // C++: `std::function<void(std::function<void()> task)> executeTask_DEPRECATED;`
  pub execute_task_deprecated: Option<Box<dyn Fn(Task) + Send + Sync>>,
  // C++: `std::function<void(std::vector<std::function<void()>> tasks)> executeTasks;`
  pub execute_tasks: Option<Box<dyn Fn(Vec<Task>) + Send + Sync>>,

  pub build_queue_items: Vec<BuildQueueItem>,
  pub mtx: Mutex<()>,
  pub cv: Condvar,
  pub ready_queue_items: Vec<usize>,
  pub processing: usize,
  pub remaining: usize,
}

unsafe impl Send for BuildQueueWorkState {}
unsafe impl Sync for BuildQueueWorkState {}
