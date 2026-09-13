use crate::{
  enums::{ir_op_kind::IrOpKind, ir_value_kind::IrValueKind},
  functions::get_cmd_value_kind::get_cmd_value_kind,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_lowering_x_64::IrLoweringX64, ir_op::IrOp, operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
};

impl IrLoweringX64 {
  pub fn mem_reg_float_op(&mut self, op: IrOp) -> OperandX64 {
    match op.kind() {
      IrOpKind::Inst => {
        let cmd = unsafe {
          let instructions = &(*self.function).instructions;
          instructions[op.index() as usize].cmd
        };
        CODEGEN_ASSERT!(get_cmd_value_kind(cmd) == IrValueKind::Float);
        OperandX64::operand_x_64_register_x_64(self.reg_op(op))
      }
      IrOpKind::Constant => {
        let double_val = self.double_op(op);
        let float_val = double_val as f32;
        unsafe { (*self.build).f32(float_val) }
      }
      _ => {
        CODEGEN_ASSERT!(false, "Unsupported operand kind");
        OperandX64::operand_x_64_register_x_64(RegisterX64::NOREG)
      }
    }
  }
}
