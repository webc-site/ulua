use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_op_kind::IrOpKind},
  functions::is_pseudo::is_pseudo,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_block::IrBlock, ir_function::IrFunction, ir_op::IrOp},
};

pub fn update_last_use_locations(function: &mut IrFunction, sorted_blocks: &[u32]) {
  for block_idx in sorted_blocks {
    let block: IrBlock = function.blocks[*block_idx as usize];

    if block.kind == IrBlockKind::Dead {
      continue;
    }

    let mut sync_arg_ops: Option<Vec<IrOp>> = None;

    if block.kind == IrBlockKind::ExitSync {
      if let Some(key) = function.block_to_vm_exit_map.find(block_idx)
        && let Some(sync_info) = function.vm_exit_info.find(key)
      {
        sync_arg_ops = Some(sync_info.arg_ops.as_slice().to_vec());
      }

      CODEGEN_ASSERT!(sync_arg_ops.is_some());
    }

    CODEGEN_ASSERT!(block.start != !0u32);
    CODEGEN_ASSERT!(block.finish != !0u32);

    for inst_idx in block.start..=block.finish {
      CODEGEN_ASSERT!((inst_idx as usize) < function.instructions.len());

      let inst = function.instructions[inst_idx as usize].clone();

      if is_pseudo(inst.cmd) {
        continue;
      }

      for op in inst.ops.as_slice() {
        if let Some(sync_arg_ops) = &sync_arg_ops
          && sync_arg_ops.iter().any(|arg_op| arg_op == op)
        {
          continue;
        }

        update_last_use_for_op(function, inst_idx, *op);
      }
    }
  }
}

fn update_last_use_for_op(function: &mut IrFunction, inst_idx: u32, op: IrOp) {
  if op.kind() == IrOpKind::Inst {
    function.instructions[op.index() as usize].last_use = inst_idx;
  } else if op.kind() == IrOpKind::Block
    && function.blocks[op.index() as usize].kind == IrBlockKind::ExitSync
  {
    let arg_ops = function
      .vm_exit_info
      .find(&inst_idx)
      .map(|sync_info| sync_info.arg_ops.as_slice().to_vec());

    if let Some(arg_ops) = arg_ops {
      for arg_op in arg_ops {
        update_last_use_for_op(function, inst_idx, arg_op);
      }
    }
  }
}
