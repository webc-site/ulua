use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64},
};

impl AssemblyBuilderX64 {
  pub fn vcvtsi2sd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    // C++: setW = (src2.cat == reg ? src2.base.size : src2.memSize) == qword.
    // A register operand carries its size in `base.size()`, NOT `memSize`
    // (which is `none` for registers), so a qword GP source needs the cat check.
    let set_w = (if src2.cat == CategoryX64::Reg {
      src2.base.size()
    } else {
      src2.mem_size
    }) == SizeX64::Qword;
    self.place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_bool_u8_u8(
      "vcvtsi2sd",
      dst,
      src1,
      src2,
      0x2a,
      set_w,
      0x0f,
      0xf2,
    );
  }
}
