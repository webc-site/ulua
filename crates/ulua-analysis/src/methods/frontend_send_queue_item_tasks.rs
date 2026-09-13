use alloc::{boxed::Box, sync::Arc, vec::Vec};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{
  build_queue_work_state::{BuildQueueWorkState, Task},
  frontend::Frontend,
};

/// Captures the raw `Frontend`/`state` pointers that each queued task needs to run
/// `performQueueItemTask`. C++ captures `this`, `state`, and `itemPos` by value into
/// the `std::function<void()>`. The default (single-threaded) executor runs each task
/// immediately on the calling thread, so the raw pointers stay valid; the `Send` marker
/// is asserted to satisfy the `Task` bound, mirroring the C++ ownership model.
struct TaskCapture {
  frontend: *mut Frontend,
  state: Arc<BuildQueueWorkState>,
  item_pos: usize,
}

unsafe impl Send for TaskCapture {}

impl Frontend {
  pub fn send_queue_item_tasks(&mut self, state: Arc<BuildQueueWorkState>, items: Vec<usize>) {
    let state_ptr = Arc::as_ptr(&state) as *mut BuildQueueWorkState;
    let frontend_ptr: *mut Frontend = self;

    let mut tasks: Vec<Task> = Vec::with_capacity(items.len());

    for &item_pos in &items {
      let queue_items = unsafe { &mut (*state_ptr).build_queue_items };
      let item = &mut queue_items[item_pos];

      LUAU_ASSERT!(!item.processing);
      item.processing = true;

      let capture = TaskCapture {
        frontend: frontend_ptr,
        state: state.clone(),
        item_pos,
      };

      tasks.push(Box::new(move || {
        let capture = capture;
        unsafe {
          (*capture.frontend).perform_queue_item_task(capture.state.clone(), capture.item_pos);
        }
      }));
    }

    unsafe {
      (*state_ptr).processing += items.len();
    }

    // C++: `state->executeTasks(std::move(tasks));`
    let execute_tasks_slot = unsafe { &(*state_ptr).execute_tasks };
    let execute_tasks = execute_tasks_slot
      .as_ref()
      .expect("BuildQueueWorkState::executeTasks must be set");
    execute_tasks(tasks);
  }
}
