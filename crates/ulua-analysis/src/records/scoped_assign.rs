#[derive(Debug)]
pub struct ScopedAssign<T> {
  pub(crate) target: *mut T,
  pub(crate) old_value: T,
}

/// # Safety
///
/// `target: *mut T` 由 `ScopedAssign::new` 从唯一的 `&mut T` 借用取得（RAII
/// 写回）。当 `T: Send` 时，移动该守卫等价于按借用检查器已许可的方式转移底层
/// `T`，`target` 亦随守卫一同移动，不产生额外别名，故 Send 可靠。
// Safety: 见上——守卫移动时 target 裸指针随行，底层 T 的独占写回权随 Send 边界
// 一起转移，不产生第二个所有者或额外别名，Send 契约成立。
unsafe impl<T: Send> Send for ScopedAssign<T> {}
/// # Safety
///
/// 当 `T: Sync` 时，共享 `&ScopedAssign<T>` 只允许经由 `target` 以与 `&T` 相同的
/// 只读方式访问底层 `T`，写回仅在 `Drop`（`&mut self`）时发生；故 Sync 可靠。
// Safety: 见上——共享引用下 target 仅用于只读访问（写回要求 &mut self 触发 Drop，
// Sync 边界不授予它）；T: Sync 保证跨线程共享 &T 本身合法，Sync 契约成立。
unsafe impl<T: Sync> Sync for ScopedAssign<T> {}
