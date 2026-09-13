#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
