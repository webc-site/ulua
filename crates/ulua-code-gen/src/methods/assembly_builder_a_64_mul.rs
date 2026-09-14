use crate::{
  enums::kind_a_64::KindA64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn mul(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    if self.log_text {
      self.log_c_char_register_a_64_register_a_64_register_a_64_i32("mul", dst, src1, src2, 0);
    }

    CODEGEN_ASSERT!(dst.kind() == KindA64::W || dst.kind() == KindA64::X);
    CODEGEN_ASSERT!(dst.kind() == src1.kind() && dst.kind() == src2.kind());

    let sf: u32 = if dst.kind() == KindA64::X {
      0x80000000
    } else {
      0
    };

    self.place(
      (dst.index() as u32)
        | ((src1.index() as u32) << 5)
        | (0b11111 << 10)
        | ((src2.index() as u32) << 16)
        | (0b0011011000u32 << 21)
        | sf,
    );
    self.commit();
  }
}
