use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::code_allocator::CodeAllocator};

impl CodeAllocator {
  pub fn allocate_new_block(&mut self, unwind_info_size: &mut usize) -> bool {
    if (self.blocks.len() + 1) * self.block_size > self.max_total_size {
      return false;
    }

    let block = self.allocate_pages(self.block_size);

    if block.is_null() {
      return false;
    }

    self.block_pos = block;
    self.block_end = unsafe { block.add(self.block_size) };
    self.blocks.push(block);

    if let Some(create_block_unwind_info) = self.create_block_unwind_info {
      let unwind_info =
        unsafe { create_block_unwind_info(self.context, block, self.block_size, unwind_info_size) };

      const K_CODE_ALIGNMENT: usize = 32;
      *unwind_info_size = (*unwind_info_size + (K_CODE_ALIGNMENT - 1)) & !(K_CODE_ALIGNMENT - 1);

      CODEGEN_ASSERT!(*unwind_info_size <= CodeAllocator::K_MAX_RESERVED_DATA_SIZE);

      if unwind_info.is_null() {
        return false;
      }

      self.unwind_infos.push(unwind_info);
    }

    true
  }
}
