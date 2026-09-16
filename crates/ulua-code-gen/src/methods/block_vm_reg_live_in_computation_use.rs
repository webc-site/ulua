use crate::{
  functions::vm_reg_op::vm_reg_op,
  records::{block_vm_reg_live_in_computation::BlockVmRegLiveInComputation, ir_op::IrOp},
};

#[unsafe(export_name = "ulua_block_vm_reg_live_in_computation_use")]
pub extern "C-unwind" fn block_vm_reg_live_in_computation_use(
  this: &mut BlockVmRegLiveInComputation<'_>,
  op: IrOp,
  offset: i32,
) {
  let reg_index = (vm_reg_op(op) + offset) as usize;
  let reg_bit = 1u64 << (reg_index % 64);
  let reg_array_index = reg_index / 64;

  if reg_array_index < 4 && (this.def_rs.regs[reg_array_index] & reg_bit) == 0 {
    this.in_rs.regs[reg_array_index] |= reg_bit;
  }
}
