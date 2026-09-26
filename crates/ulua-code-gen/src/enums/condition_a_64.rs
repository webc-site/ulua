use strum::{Display, FromRepr, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromRepr, IntoStaticStr, Display)]
#[repr(u32)]
pub enum ConditionA64 {
  #[strum(serialize = "eq")]
  Equal,
  #[strum(serialize = "ne")]
  NotEqual,
  #[strum(serialize = "cs")]
  CarrySet,
  #[strum(serialize = "cc")]
  CarryClear,
  #[strum(serialize = "mi")]
  Minus,
  #[strum(serialize = "pl")]
  Plus,
  #[strum(serialize = "vs")]
  Overflow,
  #[strum(serialize = "vc")]
  NoOverflow,
  #[strum(serialize = "hi")]
  UnsignedGreater,
  #[strum(serialize = "ls")]
  UnsignedLessEqual,
  #[strum(serialize = "ge")]
  GreaterEqual,
  #[strum(serialize = "lt")]
  Less,
  #[strum(serialize = "gt")]
  Greater,
  #[strum(serialize = "le")]
  LessEqual,
  #[strum(serialize = "al")]
  Always,
  #[strum(serialize = "nv")]
  Count,
}

impl ConditionA64 {
  pub const UNSIGNED_LESS: Self = Self::CarryClear;
  pub const UNSIGNED_GREATER_EQUAL: Self = Self::CarrySet;

  /// 条件编码值（0..=15），与底层 A64 指令编码对应。
  #[inline]
  pub const fn code(self) -> u32 {
    self as u32
  }

  /// 反汇编助记后缀，与 C++ `textForCondition` 表逐项对应。
  #[inline]
  pub fn as_str(self) -> &'static str {
    self.into()
  }

  /// 条件分支指令助记符（如 "b.eq", "b.ne"，无条件为 "b"）。
  #[inline]
  pub fn branch_mnemonic(self) -> &'static str {
    match self {
      Self::Equal => "b.eq",
      Self::NotEqual => "b.ne",
      Self::CarrySet => "b.cs",
      Self::CarryClear => "b.cc",
      Self::Minus => "b.mi",
      Self::Plus => "b.pl",
      Self::Overflow => "b.vs",
      Self::NoOverflow => "b.vc",
      Self::UnsignedGreater => "b.hi",
      Self::UnsignedLessEqual => "b.ls",
      Self::GreaterEqual => "b.ge",
      Self::Less => "b.lt",
      Self::Greater => "b.gt",
      Self::LessEqual => "b.le",
      Self::Always | Self::Count => "b",
    }
  }
}

/// 根据条件编码（含反转码 `cond ^ 1`）查找分支助记符
#[inline]
pub fn branch_mnemonic_from_code(code: u8) -> &'static str {
  ConditionA64::from_repr(code as u32).map_or("b", ConditionA64::branch_mnemonic)
}
