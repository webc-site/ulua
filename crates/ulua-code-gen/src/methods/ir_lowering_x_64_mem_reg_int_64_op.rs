use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::{
    luau_constant_value::luau_constant_value, luau_reg_value_int_64::luau_reg_value_int_64,
    vm_const_op::vm_const_op, vm_reg_op::vm_reg_op,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_lowering_x_64::IrLoweringX64, ir_op::IrOp, operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
};

impl IrLoweringX64 {
  pub fn mem_reg_int_64_op(&mut self, op: IrOp) -> OperandX64 {
    match op.kind() {
      IrOpKind::Inst => OperandX64::operand_x_64_register_x_64(self.reg_op(op)),
      IrOpKind::Constant => {
        let imm = self.int64_op(op);
        let build = unsafe { &mut *self.build };
        build.i64(imm)
      }
      IrOpKind::VmReg => luau_reg_value_int_64(vm_reg_op(op)),
      IrOpKind::VmConst => luau_constant_value(vm_const_op(op)),
      _ => {
        CODEGEN_ASSERT!(false, "Unsupported operand kind");
        OperandX64::operand_x_64_register_x_64(RegisterX64::NOREG)
      }
    }
  }
}
