use crate::{
  enums::{address_kind_a_64::AddressKindA64, kind_a_64::KindA64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::register_a_64::RegisterA64,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct AddressA64 {
  pub kind: AddressKindA64,
  pub base: RegisterA64,
  pub offset: RegisterA64,
  pub data: i32,
}

impl AddressA64 {
  pub const K_MAX_OFFSET: usize = 1023;

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
