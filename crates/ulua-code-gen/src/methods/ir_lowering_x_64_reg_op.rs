use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_lowering_x_64::IrLoweringX64, ir_op::IrOp, register_x_64::RegisterX64},
};

impl IrLoweringX64 {
  pub fn reg_op(&mut self, op: IrOp) -> RegisterX64 {
    let function = self.function;
    let inst = unsafe { (*function).inst_op(op) };

    if inst.spilled || inst.needs_reload {
      self.regs.restore(inst, false);
    }

    CODEGEN_ASSERT!(inst.reg_x64 != RegisterX64::NOREG);
    inst.reg_x64
  }
}
