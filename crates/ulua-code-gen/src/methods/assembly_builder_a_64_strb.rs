use crate::{
  enums::kind_a_64::KindA64,
  records::{
    address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
  },
};

impl AssemblyBuilderA64 {
  pub fn strb(&mut self, src: RegisterA64, dst: AddressA64) {
    debug_assert!(src.kind() == KindA64::W);

    self.place_a("strb", src, dst, 0b00_1110_0000, 0);
  }
}
