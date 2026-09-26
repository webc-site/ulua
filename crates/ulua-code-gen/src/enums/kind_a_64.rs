#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::FromRepr)]
#[repr(u8)]
pub enum KindA64 {
  None,
  W, // 32-bit GPR
  X, // 64-bit GPR
  S, // 32-bit SIMD&FP scalar
  D, // 64-bit SIMD&FP scalar
  Q, // 128-bit SIMD&FP vector
}

/// 64 位 GPR 的 `sf` 编码位（指令 bit31），`cpp/CodeGen/src/AssemblyBuilderA64.cpp`
/// 各 `place*` 函数里的 `0x80000000` 字面量单点化。
pub const K_SF64: u32 = 1 << 31;

impl KindA64 {
  /// 是否 64 位 GPR（X）：与 `dst.kind() == KindA64::X` 逐位等价。
  #[inline]
  pub const fn is_x64(self) -> bool {
    matches!(self, KindA64::X)
  }

  /// 是否 32 位 GPR（W）：`placeI12` 的 `SP`（kind=None）也按 64 位编码，
  /// 该判据与 `self != KindA64::W` 逐位等价。
  #[inline]
  pub const fn is_w32(self) -> bool {
    matches!(self, KindA64::W)
  }

  /// 指令编码 `sf` 位（bit31）：仅 64 位 GPR 置位。
  #[inline]
  pub const fn sf_bit(self) -> u32 {
    if self.is_x64() { K_SF64 } else { 0 }
  }

  /// 通用寄存器位宽：X=64，其余按 32 位窗口处理（上游同式）。
  #[inline]
  pub const fn gpr_bits(self) -> i32 {
    if self.is_x64() { 64 } else { 32 }
  }
}
