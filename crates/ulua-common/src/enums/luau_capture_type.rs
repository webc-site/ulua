#[derive(
  Debug, Clone, Copy, PartialEq, Eq, strum::FromRepr, strum::IntoStaticStr, strum::Display,
)]
#[repr(u8)]
pub enum LuauCaptureType {
  #[strum(serialize = "VAL")]
  LctVal = 0,
  #[strum(serialize = "REF")]
  LctRef = 1,
  #[strum(serialize = "UPVAL")]
  LctUpval = 2,
}

impl LuauCaptureType {
  pub const LCT_VAL: Self = Self::LctVal;
  pub const LCT_REF: Self = Self::LctRef;
  pub const LCT_UPVAL: Self = Self::LctUpval;
}
