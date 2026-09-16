use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn vfmadd213pd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_bool_u8_u8(
      "vfmadd213pd",
      dst,
      src1,
      src2,
      0xA8,
      true,
      0x0F,
      0x38,
    );
  }
}
