#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IrCondition {
  Equal,
  NotEqual,
  Less,
  NotLess,
  LessEqual,
  NotLessEqual,
  Greater,
  NotGreater,
  GreaterEqual,
  NotGreaterEqual,

  UnsignedLess,
  UnsignedLessEqual,
  UnsignedGreater,
  UnsignedGreaterEqual,

  Count,
}
