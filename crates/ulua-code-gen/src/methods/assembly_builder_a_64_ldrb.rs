use crate::{
  enums::kind_a_64::KindA64,
  records::{
    address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
  },
};

impl AssemblyBuilderA64 {
  pub fn ldrb(&mut self, dst: RegisterA64, src: AddressA64) {
    debug_assert!(dst.kind() == KindA64::W);

    self.place_a("ldrb", dst, src, 0b00_1110_0001, 0);
  }
}
