use ulua_common::macros::luau_unreachable::LUAU_UNREACHABLE;

use crate::{
  enums::kind_a_64::KindA64,
  records::{
    address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
  },
};

impl AssemblyBuilderA64 {
  pub fn ldr(&mut self, dst: RegisterA64, src: AddressA64) {
    match dst.kind() {
      KindA64::W => self.place_a("ldr", dst, src, 0b10_1110_0001, 2),
      KindA64::X => self.place_a("ldr", dst, src, 0b11_1110_0001, 3),
      KindA64::S => self.place_a("ldr", dst, src, 0b10_1111_0001, 2),
      KindA64::D => self.place_a("ldr", dst, src, 0b11_1111_0001, 3),
      KindA64::Q => self.place_a("ldr", dst, src, 0b00_1111_0011, 4),
      KindA64::None => {
        debug_assert!(false, "Unexpected register kind");
        LUAU_UNREACHABLE!();
      }
    }
  }
}
