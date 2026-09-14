use crate::{
  enums::kind_a_64::KindA64,
  records::{
    address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
  },
};

impl AssemblyBuilderA64 {
  pub fn ldp(&mut self, dst1: RegisterA64, dst2: RegisterA64, src: AddressA64) {
    debug_assert!(dst1.kind() == KindA64::X || dst1.kind() == KindA64::W);
    debug_assert!(dst1.kind() == dst2.kind());

    let is_x = dst1.kind() == KindA64::X;
    self.place_p(
      "ldp",
      dst1,
      dst2,
      src,
      0b1010_0101,
      (is_x as u8) << 1,
      if is_x { 3 } else { 2 },
    );
  }
}
