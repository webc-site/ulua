use crate::enums::size_x_64::SizeX64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RegisterX64 {
  pub(crate) bits: u8,
}

// C++ 的 `RegisterX64` 是 `{SizeX64 size; uint8_t index;}`；本 Rust 移植
// 把两者打包进一个 `bits` 字节（size 低 3 位，index 高 5 位），因此
// RegisterX64.h 的 `inline constexpr RegisterX64 name{size, index}` 全局量
// 变为关联常量。`make` 精确复现其构造函数。
impl RegisterX64 {
  pub(crate) const SIZE_MASK: u8 = 0x07;
  pub(crate) const INDEX_MASK: u8 = 0xF8;
  pub(crate) const INDEX_SHIFT: u32 = 3;

  /// C++ 的 `RegisterX64{size, index}`——把 `{size, index}` 打包进位布局。
  /// crate 内寄存器构造的唯一入口（各 emit 文件的私有 `reg` 副本已收口至此）。
  pub(crate) const fn make(size: SizeX64, index: u8) -> Self {
    RegisterX64 {
      bits: (index << Self::INDEX_SHIFT) | (size as u8),
    }
  }

  /// C++ 的 `sized(reg, size)` 助手——保持 index 不变、仅更换尺寸的编译期重打包。
  pub(crate) const fn sized(self, size: SizeX64) -> Self {
    RegisterX64 {
      bits: (self.index() << Self::INDEX_SHIFT) | (size as u8),
    }
  }

  pub const fn size(&self) -> SizeX64 {
    match SizeX64::from_repr(self.bits & Self::SIZE_MASK) {
      Some(size) => size,
      None => SizeX64::None,
    }
  }

  pub const fn index(&self) -> u8 {
    (self.bits & Self::INDEX_MASK) >> Self::INDEX_SHIFT
  }

  // RegisterX64.h:42-43——哨兵。
  pub const NOREG: RegisterX64 = Self::make(SizeX64::None, 16);
  pub const RIP: RegisterX64 = Self::make(SizeX64::None, 0);

  // RegisterX64.h:45-60——字节寄存器。
  pub const AL: RegisterX64 = Self::make(SizeX64::Byte, 0);
  pub const CL: RegisterX64 = Self::make(SizeX64::Byte, 1);
  pub const DL: RegisterX64 = Self::make(SizeX64::Byte, 2);
  pub const BL: RegisterX64 = Self::make(SizeX64::Byte, 3);
  pub const SPL: RegisterX64 = Self::make(SizeX64::Byte, 4);
  pub const BPL: RegisterX64 = Self::make(SizeX64::Byte, 5);
  pub const SIL: RegisterX64 = Self::make(SizeX64::Byte, 6);
  pub const DIL: RegisterX64 = Self::make(SizeX64::Byte, 7);
  pub const R8B: RegisterX64 = Self::make(SizeX64::Byte, 8);
  pub const R9B: RegisterX64 = Self::make(SizeX64::Byte, 9);
  pub const R10B: RegisterX64 = Self::make(SizeX64::Byte, 10);
  pub const R11B: RegisterX64 = Self::make(SizeX64::Byte, 11);
  pub const R12B: RegisterX64 = Self::make(SizeX64::Byte, 12);
  pub const R13B: RegisterX64 = Self::make(SizeX64::Byte, 13);
  pub const R14B: RegisterX64 = Self::make(SizeX64::Byte, 14);
  pub const R15B: RegisterX64 = Self::make(SizeX64::Byte, 15);

  // RegisterX64.h:62-77——dword 寄存器。
  pub const EAX: RegisterX64 = Self::make(SizeX64::Dword, 0);
  pub const ECX: RegisterX64 = Self::make(SizeX64::Dword, 1);
  pub const EDX: RegisterX64 = Self::make(SizeX64::Dword, 2);
  pub const EBX: RegisterX64 = Self::make(SizeX64::Dword, 3);
  pub const ESP: RegisterX64 = Self::make(SizeX64::Dword, 4);
  pub const EBP: RegisterX64 = Self::make(SizeX64::Dword, 5);
  pub const ESI: RegisterX64 = Self::make(SizeX64::Dword, 6);
  pub const EDI: RegisterX64 = Self::make(SizeX64::Dword, 7);
  pub const R8D: RegisterX64 = Self::make(SizeX64::Dword, 8);
  pub const R9D: RegisterX64 = Self::make(SizeX64::Dword, 9);
  pub const R10D: RegisterX64 = Self::make(SizeX64::Dword, 10);
  pub const R11D: RegisterX64 = Self::make(SizeX64::Dword, 11);
  pub const R12D: RegisterX64 = Self::make(SizeX64::Dword, 12);
  pub const R13D: RegisterX64 = Self::make(SizeX64::Dword, 13);
  pub const R14D: RegisterX64 = Self::make(SizeX64::Dword, 14);
  pub const R15D: RegisterX64 = Self::make(SizeX64::Dword, 15);

  // RegisterX64.h:79-94——qword 寄存器。
  pub const RAX: RegisterX64 = Self::make(SizeX64::Qword, 0);
  pub const RCX: RegisterX64 = Self::make(SizeX64::Qword, 1);
  pub const RDX: RegisterX64 = Self::make(SizeX64::Qword, 2);
  pub const RBX: RegisterX64 = Self::make(SizeX64::Qword, 3);
  pub const RSP: RegisterX64 = Self::make(SizeX64::Qword, 4);
  pub const RBP: RegisterX64 = Self::make(SizeX64::Qword, 5);
  pub const RSI: RegisterX64 = Self::make(SizeX64::Qword, 6);
  pub const RDI: RegisterX64 = Self::make(SizeX64::Qword, 7);
  pub const R8: RegisterX64 = Self::make(SizeX64::Qword, 8);
  pub const R9: RegisterX64 = Self::make(SizeX64::Qword, 9);
  pub const R10: RegisterX64 = Self::make(SizeX64::Qword, 10);
  pub const R11: RegisterX64 = Self::make(SizeX64::Qword, 11);
  pub const R12: RegisterX64 = Self::make(SizeX64::Qword, 12);
  pub const R13: RegisterX64 = Self::make(SizeX64::Qword, 13);
  pub const R14: RegisterX64 = Self::make(SizeX64::Qword, 14);
  pub const R15: RegisterX64 = Self::make(SizeX64::Qword, 15);

  // RegisterX64.h:96-111——xmm 寄存器。
  pub const XMM0: RegisterX64 = Self::make(SizeX64::Xmmword, 0);
  pub const XMM1: RegisterX64 = Self::make(SizeX64::Xmmword, 1);
  pub const XMM2: RegisterX64 = Self::make(SizeX64::Xmmword, 2);
  pub const XMM3: RegisterX64 = Self::make(SizeX64::Xmmword, 3);
  pub const XMM4: RegisterX64 = Self::make(SizeX64::Xmmword, 4);
  pub const XMM5: RegisterX64 = Self::make(SizeX64::Xmmword, 5);
  pub const XMM6: RegisterX64 = Self::make(SizeX64::Xmmword, 6);
  pub const XMM7: RegisterX64 = Self::make(SizeX64::Xmmword, 7);
  pub const XMM8: RegisterX64 = Self::make(SizeX64::Xmmword, 8);
  pub const XMM9: RegisterX64 = Self::make(SizeX64::Xmmword, 9);
  pub const XMM10: RegisterX64 = Self::make(SizeX64::Xmmword, 10);
  pub const XMM11: RegisterX64 = Self::make(SizeX64::Xmmword, 11);
  pub const XMM12: RegisterX64 = Self::make(SizeX64::Xmmword, 12);
  pub const XMM13: RegisterX64 = Self::make(SizeX64::Xmmword, 13);
  pub const XMM14: RegisterX64 = Self::make(SizeX64::Xmmword, 14);
  pub const XMM15: RegisterX64 = Self::make(SizeX64::Xmmword, 15);

  // RegisterX64.h:113-128——ymm 寄存器。
  pub const YMM0: RegisterX64 = Self::make(SizeX64::Ymmword, 0);
  pub const YMM1: RegisterX64 = Self::make(SizeX64::Ymmword, 1);
  pub const YMM2: RegisterX64 = Self::make(SizeX64::Ymmword, 2);
  pub const YMM3: RegisterX64 = Self::make(SizeX64::Ymmword, 3);
  pub const YMM4: RegisterX64 = Self::make(SizeX64::Ymmword, 4);
  pub const YMM5: RegisterX64 = Self::make(SizeX64::Ymmword, 5);
  pub const YMM6: RegisterX64 = Self::make(SizeX64::Ymmword, 6);
  pub const YMM7: RegisterX64 = Self::make(SizeX64::Ymmword, 7);
  pub const YMM8: RegisterX64 = Self::make(SizeX64::Ymmword, 8);
  pub const YMM9: RegisterX64 = Self::make(SizeX64::Ymmword, 9);
  pub const YMM10: RegisterX64 = Self::make(SizeX64::Ymmword, 10);
  pub const YMM11: RegisterX64 = Self::make(SizeX64::Ymmword, 11);
  pub const YMM12: RegisterX64 = Self::make(SizeX64::Ymmword, 12);
  pub const YMM13: RegisterX64 = Self::make(SizeX64::Ymmword, 13);
  pub const YMM14: RegisterX64 = Self::make(SizeX64::Ymmword, 14);
  pub const YMM15: RegisterX64 = Self::make(SizeX64::Ymmword, 15);

  #[inline]
  pub fn register_x_64_operator_eq(&self, rhs: RegisterX64) -> bool {
    self.size() == rhs.size() && self.index() == rhs.index()
  }

  #[inline]
  pub const fn register_x_64_operator_ne(&self, rhs: RegisterX64) -> bool {
    self.bits != rhs.bits
  }
}

impl Default for RegisterX64 {
  fn default() -> Self {
    Self {
      bits: SizeX64::None as u8,
    }
  }
}
