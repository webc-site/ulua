#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ConditionA64 {
  Equal,
  NotEqual,
  CarrySet,
  CarryClear,
  Minus,
  Plus,
  Overflow,
  NoOverflow,
  UnsignedGreater,
  UnsignedLessEqual,
  GreaterEqual,
  Less,
  Greater,
  LessEqual,
  Always,
  Count,
}

impl ConditionA64 {
  pub const UNSIGNED_LESS: Self = Self::CarryClear;
  pub const UNSIGNED_GREATER_EQUAL: Self = Self::CarrySet;
}
