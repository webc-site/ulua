#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::FromRepr)]
#[repr(u8)]
pub enum ConditionX64 {
  Overflow,
  NoOverflow,

  Carry,
  NoCarry,

  Below,
  BelowEqual,
  Above,
  AboveEqual,
  Equal,
  Less,
  LessEqual,
  Greater,
  GreaterEqual,

  NotBelow,
  NotBelowEqual,
  NotAbove,
  NotAboveEqual,
  NotEqual,
  NotLess,
  NotLessEqual,
  NotGreater,
  NotGreaterEqual,

  Zero,
  NotZero,

  Parity,
  NotParity,

  Count,
}

const CONDITION_CODES: [u8; 26] = [
  0x00, 0x01, 0x02, 0x03, 0x02, 0x06, 0x07, 0x03, 0x04, 0x0c, 0x0e, 0x0f, 0x0d, 0x03, 0x07, 0x06,
  0x02, 0x05, 0x0d, 0x0f, 0x0e, 0x0c, 0x04, 0x05, 0x0a, 0x0b,
];

const JCC_MNEMONICS: [&str; 26] = [
  "jo", "jno", "jc", "jnc", "jb", "jbe", "ja", "jae", "je", "jl", "jle", "jg", "jge", "jnb",
  "jnbe", "jna", "jnae", "jne", "jnl", "jnle", "jng", "jnge", "jz", "jnz", "jp", "jnp",
];

const CMOV_MNEMONICS: [&str; 26] = [
  "cmovo", "cmovno", "cmovc", "cmovnc", "cmovb", "cmovbe", "cmova", "cmovae", "cmove", "cmovl",
  "cmovle", "cmovg", "cmovge", "cmovnb", "cmovnbe", "cmovna", "cmovnae", "cmovne", "cmovnl",
  "cmovnle", "cmovng", "cmovnge", "cmovz", "cmovnz", "cmovp", "cmovnp",
];

const SETCC_MNEMONICS: [&str; 26] = [
  "seto", "setno", "setc", "setnc", "setb", "setbe", "seta", "setae", "sete", "setl", "setle",
  "setg", "setge", "setnb", "setnbe", "setna", "setnae", "setne", "setnl", "setnle", "setng",
  "setnge", "setz", "setnz", "setp", "setnp",
];

impl ConditionX64 {
  /// x86 条件码（低 4 位，0..=15），与 C++ `codeForCondition` 表完全一致。
  #[inline]
  pub const fn code(self) -> u8 {
    let idx = self as usize;
    if idx < CONDITION_CODES.len() {
      CONDITION_CODES[idx]
    } else {
      0
    }
  }

  /// 条件跳转助记符（如 "je", "jne", "jz", "jnz"）。
  #[inline]
  pub const fn jcc_mnemonic(self) -> &'static str {
    let idx = self as usize;
    if idx < JCC_MNEMONICS.len() {
      JCC_MNEMONICS[idx]
    } else {
      "jo"
    }
  }

  /// 条件传送助记符（如 "cmove", "cmovne", "cmovz"）。
  #[inline]
  pub const fn cmov_mnemonic(self) -> &'static str {
    let idx = self as usize;
    if idx < CMOV_MNEMONICS.len() {
      CMOV_MNEMONICS[idx]
    } else {
      "cmovo"
    }
  }

  /// 条件置位助记符（如 "sete", "setne", "setz"）。
  #[inline]
  pub const fn setcc_mnemonic(self) -> &'static str {
    let idx = self as usize;
    if idx < SETCC_MNEMONICS.len() {
      SETCC_MNEMONICS[idx]
    } else {
      "seto"
    }
  }
}
