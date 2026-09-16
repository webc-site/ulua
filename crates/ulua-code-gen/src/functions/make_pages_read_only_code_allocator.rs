use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::code_allocator::CodeAllocator};

pub fn make_pages_read_only(mem: *mut u8, size: usize) -> bool {
  CODEGEN_ASSERT!(CodeAllocator::align_to_page_size(mem as usize) == mem as usize);
  CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

  #[cfg(target_os = "windows")]
  {
    use core::ffi::c_void;

    use windows_sys::Win32::System::Memory::{PAGE_READONLY, VirtualProtect};

    let mut old_protect: u32 = 0;
    unsafe { VirtualProtect(mem as *const c_void, size, PAGE_READONLY, &mut old_protect) != 0 }
  }

  #[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
  {
    use core::ffi::c_void;

    unsafe extern "C" {
      fn mprotect(addr: *mut c_void, len: usize, prot: i32) -> i32;
    }

    const PROT_READ: i32 = 0x1;

    unsafe { mprotect(mem as *mut c_void, size, PROT_READ) == 0 }
  }

  #[cfg(not(any(
    target_os = "windows",
    target_os = "linux",
    target_os = "macos",
    target_os = "freebsd"
  )))]
  {
    false
  }
}
