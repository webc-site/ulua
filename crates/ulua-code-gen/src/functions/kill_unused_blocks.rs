use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::kill_ir_utils_alt_c::kill_ir_function_ir_block,
  records::{ir_block::IrBlock, ir_function::IrFunction},
};

pub fn kill_unused_blocks(function: &mut IrFunction) {
  // Start from 1 as the first block is the entry block
  let mut i = 1usize;
  while i < function.blocks.len() {
    let block: *mut IrBlock = &mut function.blocks[i];

    if unsafe { (*block).kind } != IrBlockKind::Dead && unsafe { (*block).use_count } == 0 {
      kill_ir_function_ir_block(function, unsafe { &mut *block });
    }

    i += 1;
  }
}
