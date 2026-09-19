use crate::{
  enums::kind_a_64::KindA64,
  records::{
    address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
  },
};

impl AssemblyBuilderA64 {
  pub fn ldrsh(&mut self, dst: RegisterA64, src: AddressA64) {
    debug_assert!(dst.kind() == KindA64::X || dst.kind() == KindA64::W);

    let opsize = if dst.kind() == KindA64::W {
      0b01_1110_0010 | 0b01
    } else {
      0b01_1110_0010
    };

    self.place_a("ldrsh", dst, src, opsize as u16, 1);
  }
}
