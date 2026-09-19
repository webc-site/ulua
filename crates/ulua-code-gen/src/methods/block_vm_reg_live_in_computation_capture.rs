use crate::records::block_vm_reg_live_in_computation::BlockVmRegLiveInComputation;

#[unsafe(export_name = "ulua_block_vm_reg_live_in_computation_capture")]
pub extern "C-unwind" fn block_vm_reg_live_in_computation_capture(
  this: &mut BlockVmRegLiveInComputation,
  reg: i32,
) {
  this.capture(reg);
}
