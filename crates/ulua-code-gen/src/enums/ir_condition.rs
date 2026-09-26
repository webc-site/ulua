#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::FromRepr)]
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

impl IrCondition {
  /// 判别值 → 条件的类型化安全转换：越界返回 None
  #[inline]
  pub const fn from_discriminant(value: u8) -> Option<Self> {
    Self::from_repr(value)
  }
}
