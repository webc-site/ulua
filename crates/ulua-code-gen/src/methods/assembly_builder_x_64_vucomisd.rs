use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn vucomisd(&mut self, src1: OperandX64, src2: OperandX64) {
    // C++: placeAvx("vucomisd", src1, src2, 0x2e, false, AVX_0F, AVX_66);
    // 2-operand overload — there is no vvvv source, so VEX.vvvv must be 1111
    // (placeVex with src1 = noreg). Using the 3-operand overload here wrongly
    // encoded src1 into vvvv.
    self.place_avx_c_char_operand_x_64_operand_x_64_u8_bool_u8_u8(
      "vucomisd", src1, src2, 0x2e, false, 0x0F, // AVX_0F
      0x66, // AVX_66
    );
  }
}
