#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LuauCaptureType {
  LctVal = 0,
  LctRef = 1,
  LctUpval = 2,
}

impl LuauCaptureType {
  pub const LCT_VAL: Self = Self::LctVal;
  pub const LCT_REF: Self = Self::LctRef;
  pub const LCT_UPVAL: Self = Self::LctUpval;
}
