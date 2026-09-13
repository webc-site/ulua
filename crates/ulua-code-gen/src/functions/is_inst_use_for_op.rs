use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_function::IrFunction, ir_op::IrOp},
};

pub fn is_inst_use_for_op(
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
    && let Some(sync_info) = function.vm_exit_info.find(&inst_idx)
  {
    for arg_op in sync_info.arg_ops.iter() {
      debug_assert!(arg_op.kind() == IrOpKind::Inst);

      if arg_op.index() == target_inst_idx {
        *in_vm_exit_sync = true;
        return true;
      }
    }
  }

  false
}
