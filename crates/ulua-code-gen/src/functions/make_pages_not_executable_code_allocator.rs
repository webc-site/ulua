use crate::{
  functions::make_pages_not_executable_code_allocator_alt_b::make_pages_not_executable_mut,
  records::code_allocator::CodeAllocator,
};

pub fn make_pages_not_executable(mem: *mut u8, size: usize) -> bool {
  crate::macros::codegen_assert::CODEGEN_ASSERT!(
    CodeAllocator::align_to_page_size(mem as usize) == mem as usize
  );
  crate::macros::codegen_assert::CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

  make_pages_not_executable_mut(mem, size)
}
