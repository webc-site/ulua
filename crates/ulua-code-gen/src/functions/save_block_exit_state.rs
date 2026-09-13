use alloc::vec::Vec;

use crate::records::{
  const_prop_state::ConstPropState, ir_block::IrBlock, ir_function::IrFunction,
};

pub fn save_block_exit_state(
  function: &mut IrFunction,
  block: &IrBlock,
  state: &mut ConstPropState,
) {
  let mut tags: Vec<u8> = Vec::with_capacity((state.max_reg as usize) + 1);

  for item in state.regs.iter().take(state.max_reg as usize + 1) {
    tags.push(item.tag);
  }

  let block_idx = function.get_block_index(block);
  function.block_exit_tags[block_idx as usize] = tags;
}
