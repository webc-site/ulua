use crate::{
  enums::{address_kind_a_64::AddressKindA64, kind_a_64::KindA64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{address_a_64::AddressA64, register_a_64::RegisterA64},
};

impl AddressA64 {
  pub fn address_a_64_register_a_64_i32_address_kind_a_64(
    base: RegisterA64,
    off: i32,
    kind: AddressKindA64,
  ) -> Self {
    CODEGEN_ASSERT!(base.kind() == KindA64::X || base == RegisterA64::SP);
    CODEGEN_ASSERT!(kind != AddressKindA64::Reg);

    Self {
      kind,
      base,
      offset: RegisterA64::NOREG,
      data: off,
    }
  }
}
