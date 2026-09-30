use core::ptr::NonNull;

// C++ `alignas(8)` on `data`; Rust has no field-level alignment, but aligning
// the whole struct to 8 puts `data` (after the 8-byte `next` pointer) at an
// 8-aligned offset, preserving the intent.
#[repr(C, align(8))]
#[derive(Debug)]
pub struct Page {
  /// 页链后继：`None` 即 cpp 的 `next == nullptr`（链尾），链尾判断就是 `free_pages`
  /// 的循环出口，无需再手写判空。`Option<NonNull<Page>>` 与 `*mut Page` 逐位相同
  /// （niche 复用全零），故 `#[repr(C)]` 下 `data` 偏移与 `offset_of!` 契约不变。
  pub(crate) next: Option<NonNull<Page>>,
  /// Total byte size of *this page's* heap allocation (`offset_of!(Page, data) +
  /// page_size`). C++ frees pages with `operator delete(page)`, which recovers
  /// the size from the allocator; Rust's `dealloc` requires the exact `Layout`,
  /// and a page can be over-sized for a single large allocation, so we record
  /// the real size here to free each page correctly. Not in the C++ struct — a
  /// Rust-allocator necessity, not a semantic change.
  pub(crate) alloc_size: usize,
  pub(crate) data: [u8; 8192],
}
