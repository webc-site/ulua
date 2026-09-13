use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn fmov_register_a_64_register_a_64(&mut self, dst: RegisterA64, src: RegisterA64) {
    if dst.kind() == KindA64::D && src.kind() == KindA64::D {
      self.place_r_1("fmov", dst, src, 0b0_0111_1001_1000_0001_0000);
    } else if dst.kind() == KindA64::D && src.kind() == KindA64::X {
      self.place_r_1("fmov", dst, src, 0b0_0111_1001_1001_1100_0000);
    } else if dst.kind() == KindA64::X && src.kind() == KindA64::D {
      self.place_r_1("fmov", dst, src, 0b0_0111_1001_1001_1000_0000);
    } else if dst.kind() == KindA64::S && src.kind() == KindA64::S {
      self.place_r_1("fmov", dst, src, 0b0_0111_1000_1000_0001_0000);
    } else if dst.kind() == KindA64::S && src.kind() == KindA64::W {
      self.place_r_1("fmov", dst, src, 0b0_0111_1000_1001_1100_0000);
    } else {
      panic!("Unsupported fmov kind");
    }
  }
}
