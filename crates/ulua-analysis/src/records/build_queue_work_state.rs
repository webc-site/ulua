use alloc::vec::Vec;

use parking_lot::{Condvar, Mutex, MutexGuard};

use crate::records::build_queue_item::BuildQueueItem;

/// C++: 投递给 executor 的一个 `std::function<void()>`。
///
/// Rust 端口只投递构建队列的**下标**而不是闭包：执行一个队列项需要 `&mut Frontend`
/// （`check_build_queue_item` 原地改写 `Frontend`，且 `Frontend` 经 `wire_self_pointers`
/// 自引用），任何捕获它的闭包都无法跨越线程，用 `unsafe impl Send` 强行断言即为 UB。
/// 下标不含任何借用，接收方只能通过 [`crate::type_aliases::frontend_callbacks::TaskQueue`]
/// 回调的执行器（由持有 `&mut Frontend` 的一方提供）真正执行该项。
pub type Task = usize;

/// C++: `BuildQueueWorkState` 中由 `mtx` 保护的字段。
pub struct QueueState {
  /// 队列项；仅在持锁时可变借用。
  pub build_queue_items: Vec<BuildQueueItem>,
  /// C++: `readyQueueItems`，已完成检查、等待主循环回收的项下标。
  pub ready_queue_items: Vec<usize>,
  /// 已投递、尚未回收的项数。
  pub processing: usize,
  /// 尚未回收的项数（含 `processing`）。
  pub remaining: usize,
}

/// C++: `struct BuildQueueWorkState`（去掉 `executeTasks`：executor 由调用方以可变借用
/// 传入，见 [`crate::type_aliases::frontend_callbacks::TaskQueue`]）。
///
/// 可变状态全部收进 `Mutex<QueueState>`，因此不存在「拿 `Rc`/`Arc` 的裸指针再转
/// `&mut` 就地改写」的未定义行为；`cv` 与锁配对，保持 cpp `mtx` + `cv` 的
/// 「等待就绪队列非空 / notify_one」语义。
///
/// 共享句柄是 `Rc<Self>` 而非 `Arc<Self>`：队列项携带的
/// [`ulua_config::records::config::Config`]（`ParseOptions::resume_settings` 内含
/// `*mut AstLocal`）本身就不是 `Send`，`Arc` 只会给出「可以跨线程」的假象。
pub struct BuildQueueWorkState {
  queue: Mutex<QueueState>,
  cv: Condvar,
}

impl BuildQueueWorkState {
  /// 以已构建好的队列项建立工作状态；`remaining` 初值为队列长度
  /// （C++: `state->remaining = state->buildQueueItems.size();`）。
  pub fn new(build_queue_items: Vec<BuildQueueItem>) -> Self {
    let remaining = build_queue_items.len();
    Self {
      queue: Mutex::new(QueueState {
        build_queue_items,
        ready_queue_items: Vec::new(),
        processing: 0,
        remaining,
      }),
      cv: Condvar::new(),
    }
  }

  /// 加锁访问队列状态。parking_lot 无中毒语义，与 cpp `std::mutex` 一致。
  pub fn lock(&self) -> MutexGuard<'_, QueueState> {
    self.queue.lock()
  }

  /// C++: `state->cv.notify_one();` —— 有队列项完成时唤醒等待方。
  pub fn notify_task_ready(&self) {
    self.cv.notify_one();
  }

  /// C++: `state->processing`。
  ///
  /// 返回 `Copy` 值而非守卫：若在 `if`/`while` 条件里持有临时守卫，块内再次加锁会自死锁。
  pub fn processing(&self) -> usize {
    self.lock().processing
  }

  /// C++: `state->remaining`。
  pub fn remaining(&self) -> usize {
    self.lock().remaining
  }

  /// C++: `progress(state->buildQueueItems.size() - state->remaining, state->buildQueueItems.size())`
  /// 的两个计数，一次性在同一把锁内读出。
  pub fn progress_counts(&self) -> (usize, usize) {
    let queue = self.lock();
    let total = queue.build_queue_items.len();
    (total - queue.remaining, total)
  }

  /// C++: `state->cv.wait(guard, [] { return !readyQueueItems.empty(); });`
  ///
  /// 必须在持有 [`Self::lock`] 时调用；返回时 `ready_queue_items` 非空且锁仍被持有。
  pub fn wait_for_ready_tasks<'a>(
    &self,
    mut guard: MutexGuard<'a, QueueState>,
  ) -> MutexGuard<'a, QueueState> {
    self
      .cv
      .wait_while(&mut guard, |queue| queue.ready_queue_items.is_empty());
    guard
  }
}
