use crate::{
  enums::kind_a_64::KindA64,
  records::{
    address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
  },
};

impl AssemblyBuilderA64 {
  pub fn stp(&mut self, src1: RegisterA64, src2: RegisterA64, dst: AddressA64) {
    debug_assert!(src1.kind() == KindA64::X || src1.kind() == KindA64::W);
    debug_assert!(src1.kind() == src2.kind());

    let is_x = src1.kind() == KindA64::X;
    self.place_p(
      "stp",
      src1,
      src2,
      dst,
      0b1010_0100,
      (is_x as u8) << 1,
      if is_x { 3 } else { 2 },
    );
  }
}
