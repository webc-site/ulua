use ulua_common::FFlag;

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_op_kind::IrOpKind},
  functions::is_pseudo::is_pseudo,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_function::IrFunction, ir_op::IrOp},
};

pub fn get_next_inst_use(
  function: &mut IrFunction,
  target_inst_idx: u32,
  start_inst_idx: u32,
  in_vm_exit_sync: &mut bool,
) -> u32 {
  CODEGEN_ASSERT!((start_inst_idx as usize) < function.instructions.len());

  let target_last_use = function.instructions[target_inst_idx as usize].last_use;

  for inst_idx in start_inst_idx..=target_last_use {
    let inst = function.instructions[inst_idx as usize].clone();

    if is_pseudo(inst.cmd) {
      continue;
    }

    if FFlag::LuauCodegenVmExitSync.get() {
      for op in inst.ops.as_slice() {
        if is_inst_use_for_op(function, inst_idx, target_inst_idx, *op, in_vm_exit_sync) {
          return inst_idx;
        }
      }
    } else {
      for op in inst.ops.as_slice() {
        if op.kind() == IrOpKind::Inst && op.index() == target_inst_idx {
          return inst_idx;
        }
      }
    }
  }

  CODEGEN_ASSERT!(false);
  target_last_use
}

fn is_inst_use_for_op(
  function: &mut IrFunction,
  inst_idx: u32,
  target_inst_idx: u32,
  op: IrOp,
  in_vm_exit_sync: &mut bool,
) -> bool {
  if op.kind() == IrOpKind::Inst {
    return op.index() == target_inst_idx;
  }

  if op.kind() == IrOpKind::Block
    && function.blocks[op.index() as usize].kind == IrBlockKind::ExitSync
    && let Some(sync_info) = function.vm_exit_info.find(&inst_idx)
  {
    for arg_op in sync_info.arg_ops.as_slice() {
      CODEGEN_ASSERT!(arg_op.kind() == IrOpKind::Inst);

      if arg_op.index() == target_inst_idx {
        *in_vm_exit_sync = true;
        return true;
      }
    }
  }

  false
}
