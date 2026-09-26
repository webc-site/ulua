//! Source: `Common/include/Luau/Bytecode.h`

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LuauFeedbackType {
  LftCalltarget = 0,
}

impl LuauFeedbackType {
  pub const LFT_CALLTARGET: Self = Self::LftCalltarget;
}
