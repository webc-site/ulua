#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum SubtypingVariance {
  #[default]
  Invalid,
  Covariant,
  Contravariant,
  Invariant,
}

impl SubtypingVariance {
  #[inline]
  pub const fn flipped(self) -> Self {
    match self {
      Self::Covariant => Self::Contravariant,
      Self::Contravariant => Self::Covariant,
      other => other,
    }
  }
}
