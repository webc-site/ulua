use crate::enums::kind_a_64::KindA64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RegisterA64 {
  pub(crate) bits: u8,
}

// C++ 的 `RegisterA64` 是 `{uint8_t index:5; KindA64 kind:3;}` 位域；本移植
// 把两者打包进一个 `bits` 字节（kind 低 3 位，index 高 5 位）。文件末的自由
// 函数 `reg` 复现 RegisterA64.h 中 `inline constexpr RegisterA64
// name{kind, index}` 全局量——抽取器不会把它们作为节点发出。
impl RegisterA64 {
  pub(crate) const KIND_MASK: u8 = 0x07;
  pub(crate) const INDEX_MASK: u8 = 0xF8;
  pub(crate) const INDEX_SHIFT: u32 = 3;

  pub fn kind(&self) -> KindA64 {
    KindA64::from_repr(self.bits & Self::KIND_MASK).unwrap_or(KindA64::None)
  }

  pub fn index(&self) -> u8 {
    (self.bits & Self::INDEX_MASK) >> Self::INDEX_SHIFT
  }

  /// 同宽度零寄存器：`X`→`XZR`，其余→`WZR`。
  /// `cmp`/`cmn`/`tst`/`cset` 一类「隐式目的寄存器」写 0 的编码位点唯一入口
  /// （上游 `AssemblyBuilderA64.cpp` 同处即 `kind == KindA64::x ? xzr : wzr`）。
  /// 直接比 kind 位域而不走 `kind()`，使本函数可编译期求值。
  #[inline]
  pub const fn zero_reg(self) -> RegisterA64 {
    if self.bits & Self::KIND_MASK == KindA64::X as u8 {
      Self::XZR
    } else {
      Self::WZR
    }
  }

  // RegisterA64.h 的寄存器常量。
  pub const NOREG: RegisterA64 = reg(KindA64::None, 0);
  pub const W0: RegisterA64 = reg(KindA64::W, 0);
  pub const W1: RegisterA64 = reg(KindA64::W, 1);
  pub const W2: RegisterA64 = reg(KindA64::W, 2);
  pub const W3: RegisterA64 = reg(KindA64::W, 3);
  pub const W4: RegisterA64 = reg(KindA64::W, 4);
  pub const W5: RegisterA64 = reg(KindA64::W, 5);
  pub const W7: RegisterA64 = reg(KindA64::W, 7);
  pub const W8: RegisterA64 = reg(KindA64::W, 8);
  pub const W13: RegisterA64 = reg(KindA64::W, 13);
  pub const W17: RegisterA64 = reg(KindA64::W, 17);
  pub const WZR: RegisterA64 = reg(KindA64::W, 31);
  pub const X0: RegisterA64 = reg(KindA64::X, 0);
  pub const X1: RegisterA64 = reg(KindA64::X, 1);
  pub const X2: RegisterA64 = reg(KindA64::X, 2);
  pub const X3: RegisterA64 = reg(KindA64::X, 3);
  pub const X4: RegisterA64 = reg(KindA64::X, 4);
  pub const X5: RegisterA64 = reg(KindA64::X, 5);
  pub const X6: RegisterA64 = reg(KindA64::X, 6);
  pub const X7: RegisterA64 = reg(KindA64::X, 7);
  pub const X8: RegisterA64 = reg(KindA64::X, 8);
  pub const X9: RegisterA64 = reg(KindA64::X, 9);
  pub const X16: RegisterA64 = reg(KindA64::X, 16);
  pub const X17: RegisterA64 = reg(KindA64::X, 17);
  pub const X19: RegisterA64 = reg(KindA64::X, 19);
  pub const X20: RegisterA64 = reg(KindA64::X, 20);
  pub const X21: RegisterA64 = reg(KindA64::X, 21);
  pub const X22: RegisterA64 = reg(KindA64::X, 22);
  pub const X23: RegisterA64 = reg(KindA64::X, 23);
  pub const X24: RegisterA64 = reg(KindA64::X, 24);
  pub const X25: RegisterA64 = reg(KindA64::X, 25);
  pub const X28: RegisterA64 = reg(KindA64::X, 28);
  pub const X29: RegisterA64 = reg(KindA64::X, 29);
  pub const X30: RegisterA64 = reg(KindA64::X, 30);
  pub const XZR: RegisterA64 = reg(KindA64::X, 31);
  pub const SP: RegisterA64 = reg(KindA64::None, 31);
  pub const S0: RegisterA64 = reg(KindA64::S, 0);
  pub const S1: RegisterA64 = reg(KindA64::S, 1);
  pub const S2: RegisterA64 = reg(KindA64::S, 2);
  pub const S28: RegisterA64 = reg(KindA64::S, 28);
  pub const S29: RegisterA64 = reg(KindA64::S, 29);
  pub const S30: RegisterA64 = reg(KindA64::S, 30);
  pub const D0: RegisterA64 = reg(KindA64::D, 0);
  pub const D1: RegisterA64 = reg(KindA64::D, 1);
  pub const D2: RegisterA64 = reg(KindA64::D, 2);
  pub const D3: RegisterA64 = reg(KindA64::D, 3);
  pub const D28: RegisterA64 = reg(KindA64::D, 28);
  pub const D29: RegisterA64 = reg(KindA64::D, 29);
  pub const Q0: RegisterA64 = reg(KindA64::Q, 0);
  pub const Q1: RegisterA64 = reg(KindA64::Q, 1);
  pub const Q2: RegisterA64 = reg(KindA64::Q, 2);
  pub const Q3: RegisterA64 = reg(KindA64::Q, 3);
  pub const Q28: RegisterA64 = reg(KindA64::Q, 28);
  pub const Q29: RegisterA64 = reg(KindA64::Q, 29);
  pub const Q30: RegisterA64 = reg(KindA64::Q, 30);
  pub const Q31: RegisterA64 = reg(KindA64::Q, 31);

  #[inline]
  pub const fn bits_differ(&self, rhs: RegisterA64) -> bool {
    self.bits != rhs.bits
  }
}

impl Default for RegisterA64 {
  fn default() -> Self {
    Self {
      bits: KindA64::None as u8,
    }
  }
}

/// A64 寄存器打包唯一入口：`bits = index << INDEX_SHIFT | kind`（kind 占低 3 位）。
/// 复现 C++ `RegisterA64.h` 的 `inline constexpr RegisterA64 name{kind, index}`
/// 聚合初始化语义；全 crate 15 处文件内 `const fn reg` 副本坍缩至此（`const`，
/// 寄存器常量与各调用点均在编译期折叠）。
pub(crate) const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
  RegisterA64 {
    bits: (index << RegisterA64::INDEX_SHIFT) | (kind as u8),
  }
}

impl PartialEq<RegisterA64> for &RegisterA64 {
  #[inline]
  fn eq(&self, other: &RegisterA64) -> bool {
    self.bits == other.bits
  }
}
