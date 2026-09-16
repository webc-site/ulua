use crate::records::{block_vm_reg_live_in_computation::BlockVmRegLiveInComputation, ir_op::IrOp};

#[unsafe(export_name = "ulua_block_vm_reg_live_in_computation_def")]
pub extern "C-unwind" fn block_vm_reg_live_in_computation_def(
  this: &mut BlockVmRegLiveInComputation<'_>,
  op: IrOp,
  offset: i32,
) {
  this.def(op, offset);
}
