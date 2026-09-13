use crate::{
  functions::allocate_pages_impl_code_allocator_alt_b, macros::codegen_assert::CODEGEN_ASSERT,
  records::code_allocator::CodeAllocator,
};
pub fn allocate_pages_impl(size: usize) -> *mut u8 {
  CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

  #[cfg(target_os = "windows")]
  {
    use core::ffi::c_void;

    use windows_sys::Win32::System::Memory::{
      MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE, VirtualAlloc,
    };

    unsafe {
      VirtualAlloc(
        core::ptr::null::<c_void>(),
        size,
        MEM_RESERVE | MEM_COMMIT,
        PAGE_READWRITE,
      ) as *mut u8
    }
  }

  #[cfg(not(target_os = "windows"))]
  {
    allocate_pages_impl_code_allocator_alt_b::allocate_pages_impl(size)
  }
}
