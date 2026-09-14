use crate::records::block_vm_reg_live_in_computation::BlockVmRegLiveInComputation;

#[unsafe(export_name = "ulua_block_vm_reg_live_in_computation_def_range")]
pub extern "C-unwind" fn block_vm_reg_live_in_computation_def_range(
  this: &mut BlockVmRegLiveInComputation<'_>,
  start: i32,
  count: i32,
) {
  this.def_range(start, count);
}
