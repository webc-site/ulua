use crate::enums::kind_a_64::KindA64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RegisterA64 {
  pub(crate) bits: u8,
}

// The C++ `RegisterA64` is a `{uint8_t index:5; KindA64 kind:3;}` bitfield; this
// port packs both into one `bits` byte (kind low 3, index high 5). `make`
// reproduces the `inline constexpr RegisterA64 name{kind, index}` globals from
// RegisterA64.h, which the extractor does not emit as nodes.
impl RegisterA64 {
  pub(crate) const KIND_MASK: u8 = 0x07;
  pub(crate) const INDEX_MASK: u8 = 0xF8;
  pub(crate) const INDEX_SHIFT: u32 = 3;

  const fn make(kind: KindA64, index: u8) -> Self {
    RegisterA64 {
      bits: (index << Self::INDEX_SHIFT) | (kind as u8),
    }
  }

  pub fn kind(&self) -> KindA64 {
    // C++ reads the 3-bit `kind` bitfield as a raw reinterpret. Rust forbids
    // an out-of-range enum value (a `transmute` of an invalid discriminant
    // is a NON-UNWINDING panic that aborts the whole process), so match
    // explicitly. Only 0..=5 are valid; an out-of-range encoding signals an
    // upstream bug — collapse it to `none`, which (like C++'s out-of-range
    // read) compares unequal to every real GPR/FP kind in the assert checks,
    // turning a process abort into a localized, debuggable test failure.
    match self.bits & Self::KIND_MASK {
      0 => KindA64::None,
      1 => KindA64::W,
      2 => KindA64::X,
      3 => KindA64::S,
      4 => KindA64::D,
      5 => KindA64::Q,
      _ => KindA64::None,
    }
  }

  pub fn index(&self) -> u8 {
    (self.bits & Self::INDEX_MASK) >> Self::INDEX_SHIFT
  }

  // RegisterA64.h register constants.
  pub const NOREG: RegisterA64 = Self::make(KindA64::None, 0);
  pub const W0: RegisterA64 = Self::make(KindA64::W, 0);
  pub const W1: RegisterA64 = Self::make(KindA64::W, 1);
  pub const W2: RegisterA64 = Self::make(KindA64::W, 2);
  pub const W3: RegisterA64 = Self::make(KindA64::W, 3);
  pub const W4: RegisterA64 = Self::make(KindA64::W, 4);
  pub const W5: RegisterA64 = Self::make(KindA64::W, 5);
  pub const W6: RegisterA64 = Self::make(KindA64::W, 6);
  pub const W7: RegisterA64 = Self::make(KindA64::W, 7);
  pub const W8: RegisterA64 = Self::make(KindA64::W, 8);
  pub const W9: RegisterA64 = Self::make(KindA64::W, 9);
  pub const W10: RegisterA64 = Self::make(KindA64::W, 10);
  pub const W11: RegisterA64 = Self::make(KindA64::W, 11);
  pub const W12: RegisterA64 = Self::make(KindA64::W, 12);
  pub const W13: RegisterA64 = Self::make(KindA64::W, 13);
  pub const W14: RegisterA64 = Self::make(KindA64::W, 14);
  pub const W15: RegisterA64 = Self::make(KindA64::W, 15);
  pub const W16: RegisterA64 = Self::make(KindA64::W, 16);
  pub const W17: RegisterA64 = Self::make(KindA64::W, 17);
  pub const W18: RegisterA64 = Self::make(KindA64::W, 18);
  pub const W19: RegisterA64 = Self::make(KindA64::W, 19);
  pub const W20: RegisterA64 = Self::make(KindA64::W, 20);
  pub const W21: RegisterA64 = Self::make(KindA64::W, 21);
  pub const W22: RegisterA64 = Self::make(KindA64::W, 22);
  pub const W23: RegisterA64 = Self::make(KindA64::W, 23);
  pub const W24: RegisterA64 = Self::make(KindA64::W, 24);
  pub const W25: RegisterA64 = Self::make(KindA64::W, 25);
  pub const W26: RegisterA64 = Self::make(KindA64::W, 26);
  pub const W27: RegisterA64 = Self::make(KindA64::W, 27);
  pub const W28: RegisterA64 = Self::make(KindA64::W, 28);
  pub const W29: RegisterA64 = Self::make(KindA64::W, 29);
  pub const W30: RegisterA64 = Self::make(KindA64::W, 30);
  pub const WZR: RegisterA64 = Self::make(KindA64::W, 31);
  pub const X0: RegisterA64 = Self::make(KindA64::X, 0);
  pub const X1: RegisterA64 = Self::make(KindA64::X, 1);
  pub const X2: RegisterA64 = Self::make(KindA64::X, 2);
  pub const X3: RegisterA64 = Self::make(KindA64::X, 3);
  pub const X4: RegisterA64 = Self::make(KindA64::X, 4);
  pub const X5: RegisterA64 = Self::make(KindA64::X, 5);
  pub const X6: RegisterA64 = Self::make(KindA64::X, 6);
  pub const X7: RegisterA64 = Self::make(KindA64::X, 7);
  pub const X8: RegisterA64 = Self::make(KindA64::X, 8);
  pub const X9: RegisterA64 = Self::make(KindA64::X, 9);
  pub const X10: RegisterA64 = Self::make(KindA64::X, 10);
  pub const X11: RegisterA64 = Self::make(KindA64::X, 11);
  pub const X12: RegisterA64 = Self::make(KindA64::X, 12);
  pub const X13: RegisterA64 = Self::make(KindA64::X, 13);
  pub const X14: RegisterA64 = Self::make(KindA64::X, 14);
  pub const X15: RegisterA64 = Self::make(KindA64::X, 15);
  pub const X16: RegisterA64 = Self::make(KindA64::X, 16);
  pub const X17: RegisterA64 = Self::make(KindA64::X, 17);
  pub const X18: RegisterA64 = Self::make(KindA64::X, 18);
  pub const X19: RegisterA64 = Self::make(KindA64::X, 19);
  pub const X20: RegisterA64 = Self::make(KindA64::X, 20);
  pub const X21: RegisterA64 = Self::make(KindA64::X, 21);
  pub const X22: RegisterA64 = Self::make(KindA64::X, 22);
  pub const X23: RegisterA64 = Self::make(KindA64::X, 23);
  pub const X24: RegisterA64 = Self::make(KindA64::X, 24);
  pub const X25: RegisterA64 = Self::make(KindA64::X, 25);
  pub const X26: RegisterA64 = Self::make(KindA64::X, 26);
  pub const X27: RegisterA64 = Self::make(KindA64::X, 27);
  pub const X28: RegisterA64 = Self::make(KindA64::X, 28);
  pub const X29: RegisterA64 = Self::make(KindA64::X, 29);
  pub const X30: RegisterA64 = Self::make(KindA64::X, 30);
  pub const XZR: RegisterA64 = Self::make(KindA64::X, 31);
  pub const SP: RegisterA64 = Self::make(KindA64::None, 31);
  pub const S0: RegisterA64 = Self::make(KindA64::S, 0);
  pub const S1: RegisterA64 = Self::make(KindA64::S, 1);
  pub const S2: RegisterA64 = Self::make(KindA64::S, 2);
  pub const S3: RegisterA64 = Self::make(KindA64::S, 3);
  pub const S4: RegisterA64 = Self::make(KindA64::S, 4);
  pub const S5: RegisterA64 = Self::make(KindA64::S, 5);
  pub const S6: RegisterA64 = Self::make(KindA64::S, 6);
  pub const S7: RegisterA64 = Self::make(KindA64::S, 7);
  pub const S8: RegisterA64 = Self::make(KindA64::S, 8);
  pub const S9: RegisterA64 = Self::make(KindA64::S, 9);
  pub const S10: RegisterA64 = Self::make(KindA64::S, 10);
  pub const S11: RegisterA64 = Self::make(KindA64::S, 11);
  pub const S12: RegisterA64 = Self::make(KindA64::S, 12);
  pub const S13: RegisterA64 = Self::make(KindA64::S, 13);
  pub const S14: RegisterA64 = Self::make(KindA64::S, 14);
  pub const S15: RegisterA64 = Self::make(KindA64::S, 15);
  pub const S16: RegisterA64 = Self::make(KindA64::S, 16);
  pub const S17: RegisterA64 = Self::make(KindA64::S, 17);
  pub const S18: RegisterA64 = Self::make(KindA64::S, 18);
  pub const S19: RegisterA64 = Self::make(KindA64::S, 19);
  pub const S20: RegisterA64 = Self::make(KindA64::S, 20);
  pub const S21: RegisterA64 = Self::make(KindA64::S, 21);
  pub const S22: RegisterA64 = Self::make(KindA64::S, 22);
  pub const S23: RegisterA64 = Self::make(KindA64::S, 23);
  pub const S24: RegisterA64 = Self::make(KindA64::S, 24);
  pub const S25: RegisterA64 = Self::make(KindA64::S, 25);
  pub const S26: RegisterA64 = Self::make(KindA64::S, 26);
  pub const S27: RegisterA64 = Self::make(KindA64::S, 27);
  pub const S28: RegisterA64 = Self::make(KindA64::S, 28);
  pub const S29: RegisterA64 = Self::make(KindA64::S, 29);
  pub const S30: RegisterA64 = Self::make(KindA64::S, 30);
  pub const S31: RegisterA64 = Self::make(KindA64::S, 31);
  pub const D0: RegisterA64 = Self::make(KindA64::D, 0);
  pub const D1: RegisterA64 = Self::make(KindA64::D, 1);
  pub const D2: RegisterA64 = Self::make(KindA64::D, 2);
  pub const D3: RegisterA64 = Self::make(KindA64::D, 3);
  pub const D4: RegisterA64 = Self::make(KindA64::D, 4);
  pub const D5: RegisterA64 = Self::make(KindA64::D, 5);
  pub const D6: RegisterA64 = Self::make(KindA64::D, 6);
  pub const D7: RegisterA64 = Self::make(KindA64::D, 7);
  pub const D8: RegisterA64 = Self::make(KindA64::D, 8);
  pub const D9: RegisterA64 = Self::make(KindA64::D, 9);
  pub const D10: RegisterA64 = Self::make(KindA64::D, 10);
  pub const D11: RegisterA64 = Self::make(KindA64::D, 11);
  pub const D12: RegisterA64 = Self::make(KindA64::D, 12);
  pub const D13: RegisterA64 = Self::make(KindA64::D, 13);
  pub const D14: RegisterA64 = Self::make(KindA64::D, 14);
  pub const D15: RegisterA64 = Self::make(KindA64::D, 15);
  pub const D16: RegisterA64 = Self::make(KindA64::D, 16);
  pub const D17: RegisterA64 = Self::make(KindA64::D, 17);
  pub const D18: RegisterA64 = Self::make(KindA64::D, 18);
  pub const D19: RegisterA64 = Self::make(KindA64::D, 19);
  pub const D20: RegisterA64 = Self::make(KindA64::D, 20);
  pub const D21: RegisterA64 = Self::make(KindA64::D, 21);
  pub const D22: RegisterA64 = Self::make(KindA64::D, 22);
  pub const D23: RegisterA64 = Self::make(KindA64::D, 23);
  pub const D24: RegisterA64 = Self::make(KindA64::D, 24);
  pub const D25: RegisterA64 = Self::make(KindA64::D, 25);
  pub const D26: RegisterA64 = Self::make(KindA64::D, 26);
  pub const D27: RegisterA64 = Self::make(KindA64::D, 27);
  pub const D28: RegisterA64 = Self::make(KindA64::D, 28);
  pub const D29: RegisterA64 = Self::make(KindA64::D, 29);
  pub const D30: RegisterA64 = Self::make(KindA64::D, 30);
  pub const D31: RegisterA64 = Self::make(KindA64::D, 31);
  pub const Q0: RegisterA64 = Self::make(KindA64::Q, 0);
  pub const Q1: RegisterA64 = Self::make(KindA64::Q, 1);
  pub const Q2: RegisterA64 = Self::make(KindA64::Q, 2);
  pub const Q3: RegisterA64 = Self::make(KindA64::Q, 3);
  pub const Q4: RegisterA64 = Self::make(KindA64::Q, 4);
  pub const Q5: RegisterA64 = Self::make(KindA64::Q, 5);
  pub const Q6: RegisterA64 = Self::make(KindA64::Q, 6);
  pub const Q7: RegisterA64 = Self::make(KindA64::Q, 7);
  pub const Q8: RegisterA64 = Self::make(KindA64::Q, 8);
  pub const Q9: RegisterA64 = Self::make(KindA64::Q, 9);
  pub const Q10: RegisterA64 = Self::make(KindA64::Q, 10);
  pub const Q11: RegisterA64 = Self::make(KindA64::Q, 11);
  pub const Q12: RegisterA64 = Self::make(KindA64::Q, 12);
  pub const Q13: RegisterA64 = Self::make(KindA64::Q, 13);
  pub const Q14: RegisterA64 = Self::make(KindA64::Q, 14);
  pub const Q15: RegisterA64 = Self::make(KindA64::Q, 15);
  pub const Q16: RegisterA64 = Self::make(KindA64::Q, 16);
  pub const Q17: RegisterA64 = Self::make(KindA64::Q, 17);
  pub const Q18: RegisterA64 = Self::make(KindA64::Q, 18);
  pub const Q19: RegisterA64 = Self::make(KindA64::Q, 19);
  pub const Q20: RegisterA64 = Self::make(KindA64::Q, 20);
  pub const Q21: RegisterA64 = Self::make(KindA64::Q, 21);
  pub const Q22: RegisterA64 = Self::make(KindA64::Q, 22);
  pub const Q23: RegisterA64 = Self::make(KindA64::Q, 23);
  pub const Q24: RegisterA64 = Self::make(KindA64::Q, 24);
  pub const Q25: RegisterA64 = Self::make(KindA64::Q, 25);
  pub const Q26: RegisterA64 = Self::make(KindA64::Q, 26);
  pub const Q27: RegisterA64 = Self::make(KindA64::Q, 27);
  pub const Q28: RegisterA64 = Self::make(KindA64::Q, 28);
  pub const Q29: RegisterA64 = Self::make(KindA64::Q, 29);
  pub const Q30: RegisterA64 = Self::make(KindA64::Q, 30);
  pub const Q31: RegisterA64 = Self::make(KindA64::Q, 31);
}

impl Default for RegisterA64 {
  fn default() -> Self {
    Self {
      bits: KindA64::None as u8,
    }
  }
}
