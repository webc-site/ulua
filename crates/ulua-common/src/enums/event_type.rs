//! Source: `Common/include/Luau/TimeTrace.h`

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventType {
  Enter = 0,
  Leave = 1,
  ArgName = 2,
  ArgValue = 3,
}

impl EventType {
  pub const ENTER: Self = Self::Enter;
  pub const LEAVE: Self = Self::Leave;
  pub const ARG_NAME: Self = Self::ArgName;
  pub const ARG_VALUE: Self = Self::ArgValue;
}
