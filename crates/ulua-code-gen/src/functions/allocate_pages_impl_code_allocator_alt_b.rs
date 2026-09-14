use core::{ffi::c_void, ptr::null_mut};

use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::code_allocator::CodeAllocator};

#[cfg(not(target_os = "windows"))]
unsafe extern "C" {
  fn mmap(
    addr: *mut c_void,
    len: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: isize,
  ) -> *mut c_void;
}

pub fn allocate_pages_impl(size: usize) -> *mut u8 {
  CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

  #[cfg(target_os = "windows")]
  {
    crate::functions::allocate_pages_impl_code_allocator::allocate_pages_impl(size)
  }

  #[cfg(not(target_os = "windows"))]
  unsafe {
    const PROT_READ: i32 = 0x1;
    const PROT_WRITE: i32 = 0x2;
    const MAP_PRIVATE: i32 = 0x02;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const MAP_ANON: i32 = 0x20;
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    const MAP_ANON: i32 = 0x1000;
    #[cfg(target_os = "macos")]
    const MAP_JIT: i32 = 0x0800;
    #[cfg(not(target_os = "macos"))]
    const MAP_JIT: i32 = 0;

    let result = mmap(
      null_mut(),
      size,
      PROT_READ | PROT_WRITE,
      MAP_PRIVATE | MAP_ANON | MAP_JIT,
      -1,
      0,
    );

    if result as isize == -1 {
      null_mut()
    } else {
      result.cast()
    }
  }
}
