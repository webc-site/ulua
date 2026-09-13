use core::ptr;

use crate::{
  functions::{
    flush_instruction_cache_code_allocator::flush_instruction_cache,
    make_pages_executable_code_allocator::make_pages_executable,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::code_allocator::CodeAllocator,
};

impl CodeAllocator {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn allocate_deprecated(
    &mut self,
    data: *const u8,
    data_size: usize,
    code: *const u8,
    code_size: usize,
    result: &mut *mut u8,
    result_size: &mut usize,
    result_code_start: &mut *mut u8,
  ) -> bool {
    CODEGEN_ASSERT!(true);

    const K_CODE_ALIGNMENT: usize = 32;
    let aligned_data_size = (data_size + (K_CODE_ALIGNMENT - 1)) & !(K_CODE_ALIGNMENT - 1);
    let total_size = aligned_data_size + code_size;

    if total_size > self.block_size - Self::K_MAX_RESERVED_DATA_SIZE {
      return false;
    }

    let mut start_offset = 0;

    if total_size > (self.block_end as usize - self.block_pos as usize) {
      if !self.allocate_new_block(&mut start_offset) {
        return false;
      }
      CODEGEN_ASSERT!(total_size <= (self.block_end as usize - self.block_pos as usize));
    }

    CODEGEN_ASSERT!(
      CodeAllocator::align_to_page_size(self.block_pos as usize) == self.block_pos as usize
    );

    let data_offset = start_offset + aligned_data_size - data_size;
    let code_offset = start_offset + aligned_data_size;

    if data_size > 0 {
      unsafe {
        ptr::copy_nonoverlapping(data, self.block_pos.add(data_offset), data_size);
      }
    }
    if code_size > 0 {
      unsafe {
        ptr::copy_nonoverlapping(code, self.block_pos.add(code_offset), code_size);
      }
    }

    let page_aligned_size = Self::align_to_page_size(start_offset + total_size);

    if !unsafe { make_pages_executable(self.block_pos, page_aligned_size) } {
      return false;
    }

    flush_instruction_cache(unsafe { self.block_pos.add(code_offset) }, code_size);

    *result = unsafe { self.block_pos.add(start_offset) };
    *result_size = total_size;
    *result_code_start = unsafe { self.block_pos.add(code_offset) };

    if page_aligned_size <= (self.block_end as usize - self.block_pos as usize) {
      unsafe {
        self.block_pos = self.block_pos.add(page_aligned_size);
      }
      CODEGEN_ASSERT!(
        CodeAllocator::align_to_page_size(self.block_pos as usize) == self.block_pos as usize
      );
      CODEGEN_ASSERT!(self.block_pos <= self.block_end);
    } else {
      self.block_pos = self.block_end;
    }

    true
  }
}
