use crate::records::ir_function::IrFunction;

impl IrFunction {
  pub fn materialize_restore_location(&mut self, inst_idx: u32) {
    assert!((inst_idx as usize) < self.value_restore_ops.len());
    assert!(self.value_restore_ops[inst_idx as usize].lazy);

    self.value_restore_ops[inst_idx as usize].lazy = false;
  }
}
