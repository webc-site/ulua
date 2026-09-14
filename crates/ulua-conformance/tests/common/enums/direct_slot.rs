#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
  pub fn from_u16(value: u16) -> Option<Self> {
    match value {
      1 => Some(Self::X),
      2 => Some(Self::Y),
      3 => Some(Self::Magnitude),
      4 => Some(Self::Unit),
      5 => Some(Self::Dot),
      6 => Some(Self::Min),
      7 => Some(Self::Clone),
      8 => Some(Self::Reenter),
      9 => Some(Self::Pos),
      10 => Some(Self::Normal),
      11 => Some(Self::UV),
      12 => Some(Self::Sizeof),
      _ => None,
    }
  }
}
