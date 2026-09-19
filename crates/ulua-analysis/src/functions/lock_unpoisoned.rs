use std::sync::{Mutex, MutexGuard, PoisonError};

/// 取锁并忽略「中毒」状态，等价于 cpp `std::mutex` 的行为。
///
/// cpp 侧 `std::mutex` 没有中毒语义：持锁线程抛出异常（本仓建模为 panic）后，
/// 锁会随 `unique_lock` 析构正常释放，后续取锁照常成功。Rust 的 `Mutex` 一旦在
/// 持锁期间 panic 就永久中毒，直接 `unwrap()` 会把「首个错误」级联成之后每个取
/// 锁点的 `PoisonError`，既掩盖原始诊断又可能让等待循环无限 panic。这些锁保护的
/// 都是可安全继续访问的数据（模块缓存 / 队列计数），故统一取回内层守卫。
pub fn lock_unpoisoned<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
  mutex.lock().unwrap_or_else(PoisonError::into_inner)
}
