#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IrValueKind {
  Unknown, // Used by SUBSTITUTE, argument has to be checked to get type
  None,
  Tag,
  Int,
  Int64,
  Pointer,
  Float,
  Double,
  Tvalue,

  Count,
}

/// `static constexpr unsigned kValueDwordSize[] = {0, 0, 1, 1, 2, 2, 1, 2, 4};`
/// （IrRegAllocX64.cpp:20）—— 以 `IrValueKind` 为下标，每种 kind 一个条目。
pub const K_VALUE_DWORD_SIZE: [u32; 9] = [0, 0, 1, 1, 2, 2, 1, 2, 4];
