use crate::{enums::address_kind_a_64::AddressKindA64, records::register_a_64::RegisterA64};

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
}
