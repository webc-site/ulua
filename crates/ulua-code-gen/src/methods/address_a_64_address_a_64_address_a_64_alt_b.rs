use crate::{
  enums::{address_kind_a_64::AddressKindA64, kind_a_64::KindA64},
  records::{address_a_64::AddressA64, register_a_64::RegisterA64},
};

impl AddressA64 {
  pub fn address_a_64_register_a_64_register_a_64(base: RegisterA64, offset: RegisterA64) -> Self {
    debug_assert!(base.kind() == KindA64::X);
    debug_assert!(offset.kind() == KindA64::X);

    Self {
      kind: AddressKindA64::Reg,
      base,
      offset,
      data: 0,
    }
  }
}
