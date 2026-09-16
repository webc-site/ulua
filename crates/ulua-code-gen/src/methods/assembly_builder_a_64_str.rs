use crate::{
  enums::kind_a_64::KindA64,
  records::{
    address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
  },
};

impl AssemblyBuilderA64 {
  pub fn str(&mut self, src: RegisterA64, dst: AddressA64) {
    assert!(
      src.kind() == KindA64::X
        || src.kind() == KindA64::W
        || src.kind() == KindA64::S
        || src.kind() == KindA64::D
        || src.kind() == KindA64::Q
    );

    match src.kind() {
      KindA64::W => self.place_a("str", src, dst, 0b10_11100000, 2),
      KindA64::X => self.place_a("str", src, dst, 0b11_11100000, 3),
      KindA64::S => self.place_a("str", src, dst, 0b10_11110000, 2),
      KindA64::D => self.place_a("str", src, dst, 0b11_11110000, 3),
      KindA64::Q => self.place_a("str", src, dst, 0b00_11110010, 4),
      KindA64::None => unreachable!("Unexpected register kind"),
    }
  }
}
