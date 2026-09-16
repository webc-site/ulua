use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_lowering_a_64::IrLoweringA64, ir_op::IrOp, register_a_64::RegisterA64},
};

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_reg_op(&mut self, op: IrOp) -> RegisterA64 {
    let function = self.function;
    let inst = unsafe { (*function).inst_op(op) };

    if inst.spilled || inst.needs_reload {
      self.regs.restore_reg(inst);
    }

    CODEGEN_ASSERT!((inst.reg_a64 != RegisterA64::NOREG));
    inst.reg_a64
  }
}
