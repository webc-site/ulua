use core::ptr::NonNull;

use crate::records::page::Page;

#[repr(C)]
#[derive(Debug)]
pub struct Allocator {
  /// 页链栈顶：`None` 即 cpp 的 `root == nullptr`（尚未分配任何页的空分配器，仍可继续
  /// 使用——`allocate` 会懒建首页）。可空性只出现在这一个字段上，故收 `Option<NonNull>`
  /// 让「摘链/移交」在类型上显式，不再到处判空。
  pub(crate) root: Option<NonNull<Page>>,
  pub(crate) offset: usize,
}

// Safety: Allocator 仅含 `root: Option<NonNull<Page>>` 与 `offset: usize` 纯数据、无线程本地状态；跨线程 move 只是
// 转移页链唯一所有权、无残留状态。只要转移时受管内存不被别的线程访问即成立。
unsafe impl Send for Allocator {}
// Safety: 所有变更操作（allocate/alloc 的页 bump、扩链、Drop）都要求 `&mut self`，共享 `&Allocator`
// 不暴露 `root`/`offset` 的可变访问、也无法交出重叠的页内 `&mut`；在共享期间无其它线程分配/释放即成立。
unsafe impl Sync for Allocator {}
