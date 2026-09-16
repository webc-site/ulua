use crate::records::{
  block_vm_reg_live_in_computation::BlockVmRegLiveInComputation, register_set::RegisterSet,
};

impl<'a> BlockVmRegLiveInComputation<'a> {
  pub fn block_vm_reg_live_in_computation_block_vm_reg_live_in_computation(
    def_rs: &'a mut RegisterSet,
    captured_regs: &'a mut [u64; 4],
  ) -> Self {
    let in_rs = *def_rs;
    BlockVmRegLiveInComputation {
      def_rs,
      captured_regs,
      in_rs,
    }
  }
}
