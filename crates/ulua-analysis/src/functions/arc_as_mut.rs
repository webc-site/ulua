use alloc::sync::Arc;

/// 从 `Arc` 派生可变裸指针（C++ 直译惯用法，替代散落的
/// `Arc::as_ptr(..) as *mut T`）。返回裸指针仅作为身份/写入句柄：
/// 调用方须保证对应 `Arc` 在本指针使用期间存活，且单线程独占写。
/// 经共享引用转发写入属未定义行为，故统一收敛于此惯用法。
pub(crate) fn arc_as_mut<T>(arc: &Arc<T>) -> *mut T {
  Arc::as_ptr(arc) as *mut T
}
