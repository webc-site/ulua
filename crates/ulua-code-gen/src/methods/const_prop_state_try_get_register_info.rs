use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::vm_reg_op::vm_reg_op,
  records::{const_prop_state::ConstPropState, ir_op::IrOp, register_info::RegisterInfo},
};
impl ConstPropState {
  pub fn try_get_register_info(&mut self, op: IrOp) -> Option<*mut RegisterInfo> {
    if op.kind() == IrOpKind::VmReg {
      let vm_reg = vm_reg_op(op);
      if vm_reg > self.max_reg {
        self.max_reg = vm_reg;
      }
      return Some(&mut self.regs[vm_reg as usize] as *mut RegisterInfo);
    }

    if let Some(link) = self.try_get_reg_link(op) {
      unsafe {
        let reg = (*link).reg;
        let reg_i32 = reg as i32;
        if reg_i32 > self.max_reg {
          self.max_reg = reg_i32;
        }
        return Some(&mut self.regs[reg as usize] as *mut RegisterInfo);
      }
    }

    None
  }
}
