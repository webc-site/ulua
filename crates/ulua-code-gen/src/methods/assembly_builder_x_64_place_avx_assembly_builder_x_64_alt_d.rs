use crate::{
  enums::category_x_64::CategoryX64,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, avx_op_encoding::AvxOpEncoding,
    operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};
impl AssemblyBuilderX64 {
  pub fn place_avx_imm8(
    &mut self,
    name: &str,
    dst: OperandX64,
    src1: OperandX64,
    src2: OperandX64,
    imm8: u8,
    enc: AvxOpEncoding,
  ) {
    // Avoid CODEGEN_ASSERT! because it routes through assert_call_handler expecting *const i8
    // while this invocation produces &str from stringify!(...).
    if !(dst.cat == CategoryX64::Reg) {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    if !(src1.cat == CategoryX64::Reg) {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    if !(src2.cat == CategoryX64::Reg || src2.cat == CategoryX64::Mem) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if self.log_text {
      // C++ `placeAvx(..., imm8, ...)` logs `imm8` as a trailing operand
      // (implicit `OperandX64(int32_t)` -> imm category).
      let imm_op = OperandX64::from(imm8 as i32);
      if src1.base == RegisterX64::NOREG {
        self.log_c_char_operand_x_64_operand_x_64_operand_x_64(name, src2, dst, imm_op);
      } else {
        self.log_c_char_operand_x_64_operand_x_64_operand_x_64_operand_x_64(
          name, dst, src1, src2, imm_op,
        );
      }
    }

    self.place_vex(dst, src1, src2, enc.set_w, enc.mode, enc.prefix);
    self.place(enc.code);
    self.place_reg_and_mod_reg_mem(dst, src2, 1);
    self.place_imm_8(imm8 as i32);

    self.commit();
  }

  #[inline]
  pub fn place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
    &mut self,
    name: &str,
    dst: OperandX64,
    src1: OperandX64,
    src2: OperandX64,
    imm8: u8,
    enc: AvxOpEncoding,
  ) {
    self.place_avx_imm8(name, dst, src1, src2, imm8, enc);
  }
}
