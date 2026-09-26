use alloc::vec::Vec;

use crate::records::{const_prop_state::ConstPropState, ir_function::IrFunction};

/// 目标块以索引传入，免去向 function 反查块号。
pub fn save_block_exit_state(
  function: &mut IrFunction,
  block_idx: u32,
  state: &mut ConstPropState,
) {
  let mut tags: Vec<u8> = Vec::with_capacity((state.max_reg as usize) + 1);

  for item in state.regs.iter().take(state.max_reg as usize + 1) {
    tags.push(item.tag);
  }

  function.block_exit_tags[block_idx as usize] = tags;
}
