use core::mem::size_of;

use crate::records::{ir_block::IrBlock, ir_function::IrFunction};

impl IrFunction {
  pub fn get_block_index(&self, block: &IrBlock) -> u32 {
    // Can only be called with blocks from our vector
    let block_ptr = block as *const IrBlock as usize;
    let base_ptr = self.blocks.as_ptr() as usize;
    let end_ptr = unsafe { self.blocks.as_ptr().add(self.blocks.len()) } as usize;

    if !(block_ptr >= base_ptr && block_ptr <= end_ptr) {
      panic!("IrFunction::get_block_index: block not from this function");
    }

    let offset = block_ptr - base_ptr;
    (offset / size_of::<IrBlock>()) as u32
  }
}
