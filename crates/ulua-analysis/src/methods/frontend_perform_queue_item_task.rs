use alloc::rc::Rc;

use crate::records::{build_queue_work_state::BuildQueueWorkState, frontend::Frontend};

impl Frontend {
  /// 执行一个队列项并把下标回报到就绪队列。
  ///
  /// C++: `Frontend::performQueueItemTask(state, itemPos)`。
  pub fn perform_queue_item_task(&mut self, state: Rc<BuildQueueWorkState>, item_pos: usize) {
    // C++ 在锁外原地改写 `state->buildQueueItems[itemPos]`，靠「每个任务只碰自己的项」
    // 规避竞争；这里队列项与同步状态同属一个 `Mutex`，检查期间持锁即可。
    // `check_build_queue_item` 不会回到队列（既不发任务也不读就绪队列），无重入死锁。
    {
      let mut queue = state.lock();
      self.check_build_queue_item(&mut queue.build_queue_items[item_pos]);
    }

    // C++: `try { checkBuildQueueItem(item); } catch (const InternalCompilerError&) {
    //   item.exception = std::current_exception(); }`
    // Rust 端口把 ICE 建模为 panic（与仓库其余部分一致），这里不捕获：
    // panic 直接 unwind 出 `check_queued_modules`，故 cpp 的 presence flag
    // （`item.exception`）无对应物且已删除。
    {
      let mut queue = state.lock();
      queue.ready_queue_items.push(item_pos);
    }

    state.notify_task_ready();
  }
}
