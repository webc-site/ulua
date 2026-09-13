use crate::records::{ir_call_wrapper_x_64::IrCallWrapperX64, register_x_64::RegisterX64};

impl IrCallWrapperX64 {
  pub fn rename_source_registers(&mut self, reg: RegisterX64, replacement: RegisterX64) {
    for i in 0..(self.arg_count as usize) {
      if self.args[i].candidate {
        // Raw pointers launder the aliasing of `self.args[i]` with the `&mut self`
        // receiver of `rename_register` (matches the C++ in-place rename).
        let base_ptr: *mut RegisterX64 = &mut self.args[i].source.base;
        let index_ptr: *mut RegisterX64 = &mut self.args[i].source.index;

        self.rename_register(unsafe { &mut *base_ptr }, reg, replacement);
        self.rename_register(unsafe { &mut *index_ptr }, reg, replacement);
      }
    }

    let func_base_ptr: *mut RegisterX64 = &mut self.func_op.base;
    let func_index_ptr: *mut RegisterX64 = &mut self.func_op.index;

    self.rename_register(unsafe { &mut *func_base_ptr }, reg, replacement);
    self.rename_register(unsafe { &mut *func_index_ptr }, reg, replacement);
  }
}
