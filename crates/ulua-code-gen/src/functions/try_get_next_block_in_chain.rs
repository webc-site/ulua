use core::ptr::null_mut;

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  macros::op_a::op_a,
  records::{ir_block::IrBlock, ir_function::IrFunction, ir_inst::IrInst},
};

pub fn try_get_next_block_in_chain(function: &mut IrFunction, block: &mut IrBlock) -> *mut IrBlock {
  let (is_jump, jump_op) = {
    let term_inst: &mut IrInst = &mut function.instructions[block.finish as usize];
    (term_inst.cmd == IrCmd::JUMP, op_a(term_inst))
  };

  // Follow the strict block chain
  if is_jump && jump_op.kind() == IrOpKind::Block {
    let target: &mut IrBlock = function.block_op(jump_op);

    // Has to have the same sorting key and a consecutive chain key
    if target.sortkey == block.sortkey && target.chainkey == block.chainkey + 1 {
      return target as *mut IrBlock;
    }
  }

  null_mut()
}
