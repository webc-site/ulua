use alloc::alloc::{Layout, dealloc};
use core::{mem::align_of, ptr::null_mut};

use crate::records::{allocator::Allocator, page::Page};

impl Allocator {
  /// Destructor logic for Allocator.
  /// Note: In Rust this is typically handled by a Drop implementation,
  /// but we provide the requested method for compatibility.
  pub fn allocator_allocator_dtor(&mut self) {
    unsafe {
      let mut page = self.root;
      while !page.is_null() {
        let next = (*page).next;

        // Free with the *exact* layout `allocate` used: a page can be
        // over-sized for a single large allocation, so a fixed
        // `Layout::new::<Page>()` would mismatch (dealloc with the wrong
        // size is UB). `alloc_size` was recorded at allocation time.
        let layout = Layout::from_size_align_unchecked((*page).alloc_size, align_of::<Page>());
        dealloc(page as *mut u8, layout);

        page = next;
      }
      self.root = null_mut();
    }
  }
}

/// Frees the page list on drop. Without this every parser leaks its arena pages
/// — caught by the fuzz suite's LeakSanitizer (repeated 8200-byte `Page` leaks
/// from the `compile` target).
impl Drop for Allocator {
  fn drop(&mut self) {
    self.allocator_allocator_dtor();
  }
}
