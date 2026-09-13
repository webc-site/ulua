use alloc::sync::Arc;

use crate::records::{build_queue_work_state::BuildQueueWorkState, frontend::Frontend};

impl Frontend {
  pub fn send_queue_cycle_item_task(&mut self, state: Arc<BuildQueueWorkState>) {
    let s = unsafe { &*(&*state as *const BuildQueueWorkState) };
    if let Some(i) = s.build_queue_items.iter().position(|item| !item.processing) {
      self.send_queue_item_tasks(state.clone(), vec![i]);
    }
  }
}
