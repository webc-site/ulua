use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::code_allocator::CodeAllocator};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn make_pages_executable_mut(mem: *mut u8, size: usize) -> bool {
  unsafe {
    CODEGEN_ASSERT!(CodeAllocator::align_to_page_size(mem as usize) == mem as usize);
    CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
    {
      use core::ffi::c_void;

      unsafe extern "C" {
        fn mprotect(addr: *mut c_void, len: usize, prot: i32) -> i32;
      }

      const PROT_READ: i32 = 0x1;
      const PROT_EXEC: i32 = 0x4;

      mprotect(mem as *mut c_void, size, PROT_READ | PROT_EXEC) == 0
    }
    #[cfg(target_os = "windows")]
    {
      use core::ffi::c_void;

      use windows_sys::Win32::System::Memory::{PAGE_EXECUTE_READ, VirtualProtect};

      let mut old_protect: u32 = 0;
      VirtualProtect(
        mem as *const c_void,
        size,
        PAGE_EXECUTE_READ,
        &mut old_protect as *mut u32,
      ) != 0
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
      false
    }
  }
}
