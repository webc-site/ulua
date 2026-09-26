use alloc::alloc::{Layout, dealloc};
use core::mem::align_of;

use crate::records::{allocator::Allocator, page::Page};

impl Allocator {
  /// 释放 `root` 起的整条页链（cpp `~Allocator`），由本 crate 的 `Drop` 调用。
  ///
  /// 不变量：链上每页都由 `allocate` 以 `Layout(PAGE_DATA_OFFSET + page_size,
  /// PAGE_ALIGN)` 分配、并在 `alloc_size` 记录实际大小；`next` 以 `None` 终止，
  /// 一条链只被本函数走一遍（Drop 独占）。
  fn free_pages(&mut self) {
    // 先把链顶摘下（root 立刻为 None，即 cpp 的 `root = nullptr`），再逐页前推：
    // 释放期间 self 不再持有任何页，重入或二次 Drop 只会看到空链。
    let mut page = self.root.take();
    while let Some(current) = page {
      // 用 `allocate` 当时的精确 Layout 释放：单页可能为一次超大请求超额分配，
      // 固定 `Layout::new::<Page>()` 会与分配大小不符（dealloc 传错 size 是 UB）。
      // Safety: current 由链取得，指向本分配器独占的存活 Page，next/alloc_size 是其
      // 头部普通字段（只读）；as_ptr 仅交出已拥有的地址。
      let (next, layout, raw) = unsafe {
        let cur = current.as_ptr();
        (
          (*cur).next,
          Layout::from_size_align_unchecked((*cur).alloc_size, align_of::<Page>()),
          cur.cast::<u8>(),
        )
      };

      // Safety: layout 即该页分配时使用的同一 Layout，raw 是该块首地址。
      unsafe { dealloc(raw, layout) };

      page = next;
    }
  }
}

/// Frees the page list on drop. Without this every parser leaks its arena pages
/// — caught by the fuzz suite's LeakSanitizer (repeated 8200-byte `Page` leaks
/// from the `compile` target).
impl Drop for Allocator {
  fn drop(&mut self) {
    self.free_pages();
  }
}
