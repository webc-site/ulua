use crate::{
  enums::address_kind_a_64::AddressKindA64,
  records::{address_a_64::AddressA64, register_a_64::RegisterA64},
};

pub type Mem = AddressA64;

/// The C++ `using Mem = AddressA64` is constructed as `Mem(base, off)` /
/// `Mem(base, regoffset)` (AddressA64's overloaded constructor). A Rust type
/// alias isn't callable, so this free `Mem(...)` reproduces the two-argument
/// forms (offset kind defaults to `imm`, as in the C++ ctor). The 3-argument
/// `Mem(base, off, kind)` form maps directly to the `AddressA64` constructor.
pub trait MemOffset {
  fn into_mem(self, base: RegisterA64) -> AddressA64;
}

impl MemOffset for i32 {
  fn into_mem(self, base: RegisterA64) -> AddressA64 {
    AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(base, self, AddressKindA64::Imm)
  }
}

impl MemOffset for RegisterA64 {
  fn into_mem(self, base: RegisterA64) -> AddressA64 {
    AddressA64::address_a_64_register_a_64_register_a_64(base, self)
  }
}

pub fn mem<O: MemOffset>(base: RegisterA64, off: O) -> AddressA64 {
  off.into_mem(base)
}
