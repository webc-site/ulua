use alloc::sync::Arc;

use crate::records::{build_queue_work_state::BuildQueueWorkState, frontend::Frontend};

impl Frontend {
  pub fn perform_queue_item_task(&mut self, state: Arc<BuildQueueWorkState>, item_pos: usize) {
    // SAFETY: `state` is shared across the (logically concurrent) build queue.
    // The C++ relies on `state` being a `shared_ptr` whose `BuildQueueItem`s are
    // mutated in place while synchronization is provided by `state->mtx`. We mirror
    // that by taking a `*mut` through the `Arc`.
    let state_ptr = Arc::as_ptr(&state) as *mut BuildQueueWorkState;

    // C++ wraps this in `try { ... } catch (const InternalCompilerError&)`,
    // recording the exception into `item.exception`. The Rust port models an ICE
    // as a panic (matching the rest of the codebase), so there is no catch here;
    // `item.exception` remains `None` on the success path.
    {
      let items = unsafe { &mut (*state_ptr).build_queue_items };
      let item = &mut items[item_pos];
      self.check_build_queue_item(item);
    }

    {
      let mtx = unsafe { &(*state_ptr).mtx };
      let _guard = mtx.lock().unwrap();
      let ready = unsafe { &mut (*state_ptr).ready_queue_items };
      ready.push(item_pos);
    }

    let cv = unsafe { &(*state_ptr).cv };
    cv.notify_one();
  }
}
