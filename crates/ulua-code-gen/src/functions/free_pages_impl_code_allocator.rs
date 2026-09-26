#[cfg(not(target_os = "windows"))]
use core::ffi::c_void;

use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::code_allocator::CodeAllocator};

pub fn free_pages_impl(mem: *mut u8, size: usize) {
  CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

  #[cfg(target_os = "windows")]
  {
    use core::ffi::c_void;

    use windows_sys::Win32::System::Memory::{MEM_RELEASE, VirtualFree};

    // Safety: mem 为 VirtualAlloc MEM_RESERVE 分配的基址（与分配器配对），size=0 + MEM_RELEASE
    // 是该 API 释放整保留区的合法形态；Win32 C ABI 失败返回 0，交由 CODEGEN_ASSERT 校验。
    unsafe {
      if VirtualFree(mem as *mut c_void, 0, MEM_RELEASE) == 0 {
        CODEGEN_ASSERT!(false);
      }
    }
  }
  #[cfg(not(target_os = "windows"))]
  {
    free_pages_impl_mut(mem, size);
  }
}

pub fn free_pages_impl_mut(mem: *mut u8, size: usize) {
  CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

  // Windows 路径与 base 实现逐行相同，转发消除 VirtualFree 重复体
  #[cfg(target_os = "windows")]
  {
    free_pages_impl(mem, size);
  }

  #[cfg(not(target_os = "windows"))]
  {
    unsafe extern "C" {
      fn munmap(addr: *mut c_void, len: usize) -> i32;
    }

    // Safety: mem/size 由 allocate_pages 配对产出，且 size 已断言等于页对齐值；munmap 为
    // POSIX C ABI，addr 非空、len>0 且是先前 mmap 区的页对齐前缀，满足其解除映射契约，
    // 返回码交由 CODEGEN_ASSERT 校验。
    unsafe {
      if munmap(mem as *mut c_void, size) != 0 {
        CODEGEN_ASSERT!(false);
      }
    }
  }
}
