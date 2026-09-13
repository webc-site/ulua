use alloc::alloc::{Layout, alloc, handle_alloc_error};
use core::ptr::null_mut;

use crate::records::{allocator::Allocator, page::Page};

impl Allocator {
  #[inline]
  pub fn new() -> Self {
    unsafe {
      let layout = Layout::new::<Page>();
      let ptr = alloc(layout) as *mut Page;
      if ptr.is_null() {
        handle_alloc_error(layout);
      }
      (*ptr).next = null_mut();
      // Record the allocation size so `Drop` frees this initial page with
      // the matching `Layout` (see `Page::alloc_size`).
      (*ptr).alloc_size = layout.size();
      Allocator {
        root: ptr,
        offset: 0,
      }
    }
  }
}

impl Default for Allocator {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}
