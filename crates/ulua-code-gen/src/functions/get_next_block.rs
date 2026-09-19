use crate::{
  enums::ir_block_kind::IrBlockKind,
  records::{ir_block::IrBlock, ir_function::IrFunction},
};

pub fn get_next_block<'a>(
  function: &'a mut IrFunction,
  sorted_blocks: &[u32],
  dummy: &'a mut IrBlock,
  i: usize,
) -> &'a mut IrBlock {
  for item in sorted_blocks.iter().skip(i + 1) {
    let block_idx = *item as usize;
    if function.blocks[block_idx].kind != IrBlockKind::Dead {
      return &mut function.blocks[block_idx];
    }
  }

  dummy
}
