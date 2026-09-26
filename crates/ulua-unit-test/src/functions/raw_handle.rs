//! 共享句柄 → 下游 C++ 形态裸句柄的唯一转换点。

use alloc::sync::Arc;

/// cpp 侧 `ScopePtr` / `ModulePtr` 是 `std::shared_ptr<T>`；传给下游
/// （`generalize`、`FreeType`、`linearSearchForBinding`、`parseFragment` 等）时用的就是
/// `shared_ptr::get()` 得到的**非 const** `T*`——共享可变是上游既定模型（`Scope`/`Module`
/// 在移植层里已 `unsafe impl Send + Sync` 声明该契约，`TypeArena`/`GlobalTypes` 亦是同一
/// 对象的多把别名）。
///
/// 本函数把 `Arc::as_ptr(x) as *mut T` 这条 `*const → *mut` 强转收口到一处：调用点不再
/// 各自复制 `as` 强转，也不会顺手造出 `&mut T`（那会与同一 `Arc` 的其他持有者构成非法
/// 独占借用，`unsafe impl Sync` 并不改变这点）。返回值只当句柄用：读取优先走 `Arc`，
/// 确需写入（对应上游对同一 `ScopePtr` 的非 const 使用）必须在调用点 `unsafe` 块里按
/// "单线程、顺序执行、无并发借用" 的契约进行。
#[inline]
pub(crate) fn raw_handle<T>(shared: &Arc<T>) -> *mut T {
  Arc::as_ptr(shared).cast_mut()
}
