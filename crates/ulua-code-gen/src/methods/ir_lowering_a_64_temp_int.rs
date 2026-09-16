use crate::{
  enums::{ir_op_kind::IrOpKind, kind_a_64::KindA64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_lowering_a_64::IrLoweringA64, ir_op::IrOp, register_a_64::RegisterA64},
};

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_temp_int(&mut self, op: IrOp) -> RegisterA64 {
    match op.kind() {
      IrOpKind::Inst => self.ir_lowering_a_64_reg_op(op),
      IrOpKind::Constant => {
        let temp = self.regs.alloc_temp(KindA64::W);
        let int_val = self.ir_lowering_a_64_int_op(op);
        unsafe { (*self.build).mov_register_a_64_i32(temp, int_val) };
        temp
      }
      _ => {
        CODEGEN_ASSERT!(false, "Unsupported instruction form");
        RegisterA64::NOREG
      }
    }
  }
}
