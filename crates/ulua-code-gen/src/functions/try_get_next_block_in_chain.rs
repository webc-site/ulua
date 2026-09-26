use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  macros::op_a::op_a,
  records::{ir_function::IrFunction, ir_inst::IrInst},
};

/// 链式后继块：以块索引表达（None 表示无链式后继），替代原裸指针返回。
pub fn try_get_next_block_idx_in_chain(function: &mut IrFunction, block_idx: u32) -> Option<u32> {
  let (is_jump, jump_op, sortkey, chainkey) = {
    let block = &function.blocks[block_idx as usize];
    let term_inst: &mut IrInst = &mut function.instructions[block.finish as usize];
    (
      term_inst.cmd == IrCmd::JUMP,
      op_a(term_inst),
      block.sortkey,
      block.chainkey,
    )
  };

  // 沿严格 block 链前进
  if is_jump && jump_op.kind() == IrOpKind::Block {
    let target_idx = jump_op.index();
    let target = &mut function.blocks[target_idx as usize];

    // 必须有相同 sorting key 和连续的 chain key
    if target.sortkey == sortkey && target.chainkey == chainkey + 1 {
      return Some(target_idx);
    }
  }

  None
}
