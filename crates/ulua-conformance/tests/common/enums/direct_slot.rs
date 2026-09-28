#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::FromRepr)]
pub enum DirectSlot {
  X = 1,
  Y,
  Magnitude,
  Unit,
  Dot,
  Min,
  Clone,
  Reenter,
  Pos,
  Normal,
  UV,
  Sizeof,
}

impl DirectSlot {
  #[inline]
  pub fn from_u16(value: u16) -> Option<Self> {
    Self::from_repr(value)
  }
}
