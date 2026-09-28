#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AvxOpEncoding {
  pub code: u8,
  pub set_w: bool,
  pub mode: u8,
  pub prefix: u8,
}

impl AvxOpEncoding {
  pub const VCMP: Self = Self {
    code: 0xc2,
    set_w: false,
    mode: 0x0F,
    prefix: 0xF2,
  };
  pub const VBLENDVPD: Self = Self {
    code: 0x4b,
    set_w: false,
    mode: 0x3A,
    prefix: 0x66,
  };
  pub const VBLENDVPS: Self = Self {
    code: 0x4a,
    set_w: false,
    mode: 0x4a,
    prefix: 0x66,
  };
  pub const VDPPS: Self = Self {
    code: 0x40,
    set_w: false,
    mode: 0x3A,
    prefix: 0x66,
  };
  pub const VPINSRD: Self = Self {
    code: 0x22,
    set_w: false,
    mode: 0x3A,
    prefix: 0x66,
  };
  pub const VROUNDSD: Self = Self {
    code: 0x0b,
    set_w: false,
    mode: 0x3A,
    prefix: 0x66,
  };
  pub const VPSHUTFPS: Self = Self {
    code: 0xc6,
    set_w: false,
    mode: 0x0F,
    prefix: 0x00,
  };
}
