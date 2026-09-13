use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_op_kind::IrOpKind},
  records::{ir_function::IrFunction, ir_op::IrOp},
};

pub fn update_last_use_for_op(function: &mut IrFunction, inst_idx: u32, op: IrOp) {
  if op.kind() == IrOpKind::Inst {
    function.instructions[op.index() as usize].last_use = inst_idx;
  } else if op.kind() == IrOpKind::Block {
    let block = &function.blocks[op.index() as usize];
    if block.kind == IrBlockKind::ExitSync
      && let Some(sync_info) = function.vm_exit_info.find(&inst_idx)
    {
      let arg_ops = sync_info.arg_ops.clone();
      for arg_op in arg_ops.iter() {
        update_last_use_for_op(function, inst_idx, *arg_op);
      }
    }
  }
}
