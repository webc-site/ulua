use crate::{
  enums::{ir_op_kind::IrOpKind, ir_value_kind::IrValueKind, size_x_64::SizeX64},
  functions::get_cmd_value_kind::get_cmd_value_kind,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_lowering_x_64::IrLoweringX64, ir_op::IrOp, operand_x_64::OperandX64,
    register_x_64::RegisterX64, scoped_reg_x_64::ScopedRegX64,
  },
};

impl IrLoweringX64 {
  pub fn store_float(&mut self, dst: OperandX64, src: IrOp) {
    unsafe {
      if src.kind() == IrOpKind::Constant {
        let mut tmp = ScopedRegX64 {
          owner: &mut self.regs,
          reg: RegisterX64::NOREG,
        };
        tmp.alloc(SizeX64::Xmmword);
        let float_val = self.double_op(src) as f32;
        let tmp_reg = tmp.reg;
        let imm = (*self.build).f32(float_val);
        (*self.build).place_avx_c_char_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
          "vmovss",
          OperandX64::reg(tmp_reg),
          imm,
          0x10,
          0x11,
          false,
          0b0000,
          0b0010,
        );
        (*self.build).place_avx_c_char_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
          "vmovss",
          dst,
          OperandX64::reg(tmp_reg),
          0x10,
          0x11,
          false,
          0b0000,
          0b0010,
        );
      } else if src.kind() == IrOpKind::Inst {
        let inst = (*self.function).inst_op(src);
        CODEGEN_ASSERT!(get_cmd_value_kind(inst.cmd) == IrValueKind::Float);
        (*self.build).place_avx_c_char_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
          "vmovss",
          dst,
          OperandX64::reg(self.reg_op(src)),
          0x10,
          0x11,
          false,
          0b0000,
          0b0010,
        );
      } else {
        CODEGEN_ASSERT!(false, "Unsupported instruction form");
      }
    }
  }
}
