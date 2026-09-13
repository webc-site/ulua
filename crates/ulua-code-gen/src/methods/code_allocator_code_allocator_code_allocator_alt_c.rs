use core::ptr::null_mut;

use ulua_common::FFlag;

use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::code_allocator::CodeAllocator};

impl CodeAllocator {
  pub fn destroy(&mut self) {
    if self.destroyed {
      return;
    }

    self.destroyed = true;

    if let Some(destroy_block_unwind_info_fn) = self.destroy_block_unwind_info {
      for unwind_info in &self.unwind_infos {
        unsafe {
          destroy_block_unwind_info_fn(self.context, *unwind_info);
        }
      }
    }

    if FFlag::LuauCodegenFreeBlocks.get() {
      CODEGEN_ASSERT!(self.live_allocations == 0);
    }

    for block in &self.blocks {
      self.free_pages(*block, self.block_size);
    }

    self.unwind_infos.clear();
    self.blocks.clear();
    self.block_pos = null_mut();
    self.block_end = null_mut();
  }
}
