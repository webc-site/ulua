#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct UnwindCodeWin {
  pub offset: u8,
  pub opcode_opinfo: u8,
}

impl UnwindCodeWin {
  #[inline]
  pub fn get_opcode(&self) -> u8 {
    self.opcode_opinfo & 0x0F
  }

  #[inline]
  pub fn set_opcode(&mut self, value: u8) {
    self.opcode_opinfo = (self.opcode_opinfo & 0xF0) | (value & 0x0F);
  }

  #[inline]
  pub fn get_opinfo(&self) -> u8 {
    (self.opcode_opinfo >> 4) & 0x0F
  }

  #[inline]
  pub fn set_opinfo(&mut self, value: u8) {
    self.opcode_opinfo = (self.opcode_opinfo & 0x0F) | ((value & 0x0F) << 4);
  }
}
