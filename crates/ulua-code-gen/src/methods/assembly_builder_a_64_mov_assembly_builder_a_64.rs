use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn mov_register_a_64_register_a_64(&mut self, dst: RegisterA64, src: RegisterA64) {
    if dst.kind() != KindA64::Q {
      debug_assert!(dst.kind() == KindA64::W || dst.kind() == KindA64::X || dst == RegisterA64::SP);
      debug_assert!(
        dst.kind() == src.kind()
          || (dst.kind() == KindA64::X && src == RegisterA64::SP)
          || (dst == RegisterA64::SP && src.kind() == KindA64::X)
      );

      if dst == RegisterA64::SP || src == RegisterA64::SP {
        self.place_r_1("mov", dst, src, 0b0_0100_0100_0000_0000_0000);
      } else {
        self.place_sr_2("mov", dst, src, 0b01_01010, 0);
      }
    } else {
      debug_assert!(dst.kind() == src.kind());

      self.place_r_1(
        "mov",
        dst,
        src,
        0b1_0011_1010_1000_0000_0111 | ((src.index() as u32) << 6),
      );
    }
  }
}
