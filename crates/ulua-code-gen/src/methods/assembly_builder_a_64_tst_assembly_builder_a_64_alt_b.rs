use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn tst_register_a_64_u32(&mut self, src1: RegisterA64, src2: u32) {
    let dst = if src1.kind() == KindA64::X {
      RegisterA64::XZR
    } else {
      RegisterA64::WZR
    };

    self.place_bm("tst", dst, src1, src2, 0b11_100100);
  }
}
