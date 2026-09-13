#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct UnwindInfoWin {
  pub version_flags: u8,
  pub prologsize: u8,
  pub unwindcodecount: u8,
  pub framereg_frameregoff: u8,
}

impl UnwindInfoWin {
  #[inline]
  pub fn get_version(&self) -> u8 {
    self.version_flags & 0x07
  }

  #[inline]
  pub fn set_version(&mut self, value: u8) {
    self.version_flags = (self.version_flags & 0xF8) | (value & 0x07);
  }

  #[inline]
  pub fn get_flags(&self) -> u8 {
    (self.version_flags >> 3) & 0x1F
  }

  #[inline]
  pub fn set_flags(&mut self, value: u8) {
    self.version_flags = (self.version_flags & 0x07) | ((value & 0x1F) << 3);
  }

  #[inline]
  pub fn get_framereg(&self) -> u8 {
    self.framereg_frameregoff & 0x0F
  }

  #[inline]
  pub fn set_framereg(&mut self, value: u8) {
    self.framereg_frameregoff = (self.framereg_frameregoff & 0xF0) | (value & 0x0F);
  }

  #[inline]
  pub fn get_frameregoff(&self) -> u8 {
    (self.framereg_frameregoff >> 4) & 0x0F
  }

  #[inline]
  pub fn set_frameregoff(&mut self, value: u8) {
    self.framereg_frameregoff = (self.framereg_frameregoff & 0x0F) | ((value & 0x0F) << 4);
  }
}
