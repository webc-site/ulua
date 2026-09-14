use crate::records::code_allocator::CodeAllocator;

impl CodeAllocator {
  pub fn align_to_page_size(size: usize) -> usize {
    #[cfg(target_os = "windows")]
    let page_size = 4096;

    #[cfg(not(target_os = "windows"))]
    let page_size = unsafe {
      unsafe extern "C" {
        fn getpagesize() -> i32;
      }

      getpagesize() as usize
    };

    (size + page_size - 1) & !(page_size - 1)
  }
}
