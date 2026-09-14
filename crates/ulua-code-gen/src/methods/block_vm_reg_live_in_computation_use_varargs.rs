use crate::{
  functions::require_variadic_sequence::require_variadic_sequence,
  records::block_vm_reg_live_in_computation::BlockVmRegLiveInComputation,
};

impl<'a> BlockVmRegLiveInComputation<'a> {
  pub fn block_vm_reg_live_in_computation_use_varargs(&mut self, vararg_start: u8) {
    require_variadic_sequence(&mut self.in_rs, self.def_rs, vararg_start);

    // Variadic sequence has been consumed
    self.def_rs.vararg_seq = false;
    self.def_rs.vararg_start = 0;
  }
}
