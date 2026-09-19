use alloc::{rc::Rc, vec::Vec};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    build_queue_work_state::{BuildQueueWorkState, Task},
    frontend::Frontend,
  },
  type_aliases::frontend_callbacks::TaskQueue,
};

impl Frontend {
  /// 标记这些队列项为处理中，并把它们交给调用方的派发方。
  ///
  /// C++: `Frontend::sendQueueItemTasks(state, items)` —— cpp 在这里把
  /// `[this, state, itemPos] { performQueueItemTask(state, itemPos); }` 打包成
  /// `std::function<void()>` 交给 `state->executeTasks`。Rust 端口投递的是下标，
  /// 执行器（`run`）由本方法提供，只能在当前持有 `&mut Frontend` 的栈帧内被调用。
  pub fn send_queue_item_tasks(
    &mut self,
    state: Rc<BuildQueueWorkState>,
    items: Vec<usize>,
    execute_tasks: &mut TaskQueue,
  ) {
    let mut tasks: Vec<Task> = Vec::with_capacity(items.len());

    {
      let mut queue = state.lock();

      for &item_pos in &items {
        let item = &mut queue.build_queue_items[item_pos];

        LUAU_ASSERT!(!item.processing);
        item.processing = true;

        tasks.push(item_pos);
      }

      queue.processing += items.len();
    }

    // 锁必须在派发前释放：`run` 会进入 `perform_queue_item_task`，后者自己加锁。
    execute_tasks(tasks, &mut |task| {
      self.perform_queue_item_task(state.clone(), task)
    });
  }
}
