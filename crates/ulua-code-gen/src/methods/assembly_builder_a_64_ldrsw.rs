use crate::{
  enums::kind_a_64::KindA64,
  records::{
    address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
  },
};

impl AssemblyBuilderA64 {
  pub fn ldrsw(&mut self, dst: RegisterA64, src: AddressA64) {
    debug_assert!(dst.kind() == KindA64::X);

    self.place_a("ldrsw", dst, src, 0b10_1110_0010, 2);
  }
}
