use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};
impl AssemblyBuilderA64 {
  pub fn cmn(&mut self, src1: RegisterA64, src2: u16) {
    let dst = if src1.kind() == KindA64::X {
      RegisterA64::XZR
    } else {
      RegisterA64::WZR
    };

    self.place_i12("cmn", dst, src1, src2 as i32, 0b01_10001);
  }
}
