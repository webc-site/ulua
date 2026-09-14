use crate::{
  functions::vm_reg_op::vm_reg_op,
  records::{const_prop_state::ConstPropState, ir_op::IrOp, register_info::RegisterInfo},
};

impl ConstPropState {
  pub fn invalidate_tag(&mut self, reg_op: IrOp) {
    let reg = vm_reg_op(reg_op);
    if reg > self.max_reg {
      self.max_reg = reg;
    }

    let reg_idx = reg as usize;
    let reg_ptr: *mut RegisterInfo = &mut self.regs[reg_idx];
    unsafe {
      self.invalidate_register_info_bool_bool(&mut *reg_ptr, true, false);
    }
  }
}
