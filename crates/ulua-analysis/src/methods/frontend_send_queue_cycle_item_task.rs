use alloc::{rc::Rc, vec::Vec};

use crate::{
  records::{build_queue_work_state::BuildQueueWorkState, frontend::Frontend},
  type_aliases::frontend_callbacks::TaskQueue,
};

impl Frontend {
  /// 图上存在环时，随便挑一个还没处理的项先跑起来。
  ///
  /// C++: `Frontend::sendQueueCycleItemTask(state)`。
  pub fn send_queue_cycle_item_task(
    &mut self,
    state: Rc<BuildQueueWorkState>,
    execute_tasks: &mut TaskQueue,
  ) {
    // 语句结束即释放锁，`send_queue_item_tasks` 里会重新加锁。
    let pending = state
      .lock()
      .build_queue_items
      .iter()
      .position(|item| !item.processing);

    if let Some(item_pos) = pending {
      self.send_queue_item_tasks(state, Vec::from([item_pos]), execute_tasks);
    }
  }
}
