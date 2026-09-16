use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::{luau_reg_value_int::luau_reg_value_int, vm_reg_op::vm_reg_op},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_lowering_x_64::IrLoweringX64, ir_op::IrOp, operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
};

impl IrLoweringX64 {
  pub fn mem_reg_uint_op(&mut self, op: IrOp) -> OperandX64 {
    match op.kind() {
      IrOpKind::Inst => {
        let reg = self.reg_op(op);
        OperandX64::operand_x_64_register_x_64(reg)
      }
      IrOpKind::Constant => {
        let imm = self.int_op(op) as u32;
        OperandX64::operand_x_64_i32(imm as i32)
      }
      IrOpKind::VmReg => {
        let ri = vm_reg_op(op);
        luau_reg_value_int(ri)
      }
      _ => {
        CODEGEN_ASSERT!(false);
        OperandX64::operand_x_64_register_x_64(RegisterX64::NOREG)
      }
    }
  }
}
