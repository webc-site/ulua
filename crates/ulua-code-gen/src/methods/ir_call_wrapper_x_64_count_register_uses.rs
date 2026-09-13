use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;

impl IrCallWrapperX64 {
  pub fn count_register_uses(&mut self) {
    for i in 0..self.arg_count {
      let arg = unsafe { &mut *self.args.as_mut_ptr().add(i as usize) };
      self.add_register_use(arg.source.base);
      self.add_register_use(arg.source.index);
    }

    self.add_register_use(self.func_op.base);
    self.add_register_use(self.func_op.index);
  }
}
