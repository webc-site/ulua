#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ABIX64 {
  #[default]
  Windows,
  SystemV,
}

impl ABIX64 {
  pub const WINDOWS: ABIX64 = ABIX64::Windows;
  pub const SYSTEM_V: ABIX64 = ABIX64::SystemV;
}
