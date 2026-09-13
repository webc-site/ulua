#[cfg(any(target_os = "linux", target_os = "macos"))]
use core::ffi::c_void;

use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::code_allocator::CodeAllocator};

#[cfg(any(target_os = "linux", target_os = "macos"))]
unsafe extern "C" {
  fn mprotect(addr: *mut c_void, len: usize, prot: i32) -> i32;
}

pub fn make_pages_not_executable_mut(mem: *mut u8, size: usize) -> bool {
  CODEGEN_ASSERT!(CodeAllocator::align_to_page_size(mem as usize) == mem as usize);
  CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

  #[cfg(any(target_os = "linux", target_os = "macos"))]
  {
    const PROT_READ: i32 = 0x1;
    const PROT_WRITE: i32 = 0x2;

    unsafe { mprotect(mem as *mut c_void, size, PROT_READ | PROT_WRITE) == 0 }
  }

  // Windows was an unimplemented `false` stub, so CodeAllocator::deallocate (which
  // CODEGEN_ASSERT!s this result) aborted on every code-block free with 0x80000003
  // on Windows. Mirror the executable path's VirtualProtect, flipping protection
  // back to PAGE_READWRITE.
  #[cfg(target_os = "windows")]
  {
    use core::ffi::c_void;

    use windows_sys::Win32::System::Memory::{PAGE_READWRITE, VirtualProtect};

    let mut old_protect: u32 = 0;
    unsafe {
      VirtualProtect(
        mem as *const c_void,
        size,
        PAGE_READWRITE,
        &mut old_protect as *mut u32,
      ) != 0
    }
  }

  #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
  {
    false
  }
}
