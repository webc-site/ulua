use crate::{
  enums::address_kind_a_64::AddressKindA64,
  records::{address_a_64::AddressA64, register_a_64::RegisterA64},
};

// C++ 的 `using Mem = AddressA64` 以 `Mem(base, off)` /
/// `Mem(base, regoffset)` 形式构造（AddressA64 的重载构造函数）。Rust
/// 的类型别名不可调用，故这个自由函数 `Mem(...)` 复现两种双参形式
/// （offset kind 默认 `imm`，与 C++ ctor 一致）。三参
/// `Mem(base, off, kind)` 形式直接对应 `AddressA64` 构造函数。
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
