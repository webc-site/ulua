use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;

impl IrCallWrapperX64 {
  pub fn count_register_uses(&mut self) {
    // 快照各参数的寄存器字段（RegisterX64 是 Copy），规避与 add_register_use 可变借用冲突，去除原 unsafe
    for i in 0..self.arg_count as usize {
      let (base, index) = (self.args[i].source.base, self.args[i].source.index);
      self.add_register_use(base);
      self.add_register_use(index);
    }

    self.add_register_use(self.func_op.base);
    self.add_register_use(self.func_op.index);
  }
}
