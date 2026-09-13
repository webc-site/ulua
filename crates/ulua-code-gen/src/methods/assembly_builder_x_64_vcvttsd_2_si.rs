use crate::{
  enums::size_x_64::SizeX64,
  records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64},
};
impl AssemblyBuilderX64 {
  pub fn vcvttsd2si(&mut self, dst: OperandX64, src: OperandX64) {
    self.place_avx_c_char_operand_x_64_operand_x_64_u8_bool_u8_u8(
      "vcvttsd2si",
      dst,
      src,
      0x2c,
      dst.base.size() == SizeX64::Qword,
      0x0F, // AVX_0F
      0xF2, // AVX_F2
    );
  }
}
