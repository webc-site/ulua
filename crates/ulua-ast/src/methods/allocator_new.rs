use alloc::alloc::{Layout, alloc, handle_alloc_error};
use core::ptr::NonNull;

use crate::records::{allocator::Allocator, page::Page};

impl Allocator {
  #[inline]
  pub fn new() -> Self {
    let layout = Layout::new::<Page>();

    // Safety: alloc 以 Layout::new::<Page>()（大小非零、对齐满足 Page）申请，返回值
    // 要么为空（立即 handle_alloc_error 中止）、要么是块首地址，故 `ptr` 必为非空且按
    // Page 对齐的已分配区域，指向本分配器独占拥有的 Page 内存。
    let ptr = match NonNull::new(unsafe { alloc(layout) }.cast::<Page>()) {
      Some(ptr) => ptr,
      None => handle_alloc_error(layout),
    };

    // Safety: ptr 是上面刚分配成功的块，next/alloc_size 是该区域内的普通字段，先写后
    // 用作普通数据，无未初始化读取；首页是链尾，故 next 写 None（cpp 的 `nullptr`）。
    unsafe {
      let page = ptr.as_ptr();
      (*page).next = None;
      // Record the allocation size so `Drop` frees this initial page with
      // the matching `Layout` (see `Page::alloc_size`).
      (*page).alloc_size = layout.size();
    }

    Allocator {
      root: Some(ptr),
      offset: 0,
    }
  }
}

impl Default for Allocator {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}
