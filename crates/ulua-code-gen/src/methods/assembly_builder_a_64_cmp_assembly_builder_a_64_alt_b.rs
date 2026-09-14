use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn cmp_register_a_64_u16(&mut self, src1: RegisterA64, src2: u16) {
    let xzr = RegisterA64::XZR;
    let wzr = RegisterA64::WZR;

    let dst = if src1.kind() == KindA64::X { xzr } else { wzr };

    self.place_i12("cmp", dst, src1, src2 as i32, 0b11_10001);
  }
}
