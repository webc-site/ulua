use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64},
};

impl AssemblyBuilderX64 {
  pub fn vcvtss2sd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    if src2.cat == CategoryX64::Reg {
      CODEGEN_ASSERT!(src2.base.size() == SizeX64::Xmmword);
    } else {
      CODEGEN_ASSERT!(src2.mem_size == SizeX64::Dword);
    }

    self.place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_bool_u8_u8(
      "vcvtss2sd",
      dst,
      src1,
      src2,
      0x5a,
      false,
      0b0001,
      0b10,
    );
  }
}
