use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn vmovaps(&mut self, dst: OperandX64, src: OperandX64) {
    // C++: placeAvx("vmovaps", dst, src, 0x28, 0x29, false, AVX_0F, AVX_NP);
    // This is the load/store overload (with `coderev` 0x29) so a memory
    // destination is handled by swapping operands; the no-coderev overload
    // trips the `dst.cat == reg` assert on the store form.
    self.place_avx_c_char_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
      "vmovaps", dst, src, 0x28, 0x29, false, 0x0F, 0x00,
    );
  }
}
