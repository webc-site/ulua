use crate::{
  functions::free_pages_impl_code_allocator_alt_b::free_pages_impl_mut,
  macros::codegen_assert::CODEGEN_ASSERT, records::code_allocator::CodeAllocator,
};

pub fn free_pages_impl(mem: *mut u8, size: usize) {
  CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

  #[cfg(target_os = "windows")]
  {
    use core::ffi::c_void;

    use windows_sys::Win32::System::Memory::{MEM_RELEASE, VirtualFree};

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
