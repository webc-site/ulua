use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_op::IrOp, ir_value_location_tracking::IrValueLocationTracking},
};

impl IrValueLocationTracking {
  pub fn invalidate_restore_vm_regs(&mut self, start: i32, count: i32) {
    let mut end = if count == -1 { 255 } else { start + count };

    if end > self.max_reg {
      end = self.max_reg;
    }

    let mut reg = start;
    while reg <= end {
      self.invalidate_restore_op(
        IrOp::ir_op_ir_op_kind_u32(IrOpKind::VmReg, reg as u32),
        false,
      );
      reg += 1;
    }
  }
}
