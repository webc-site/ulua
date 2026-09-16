use crate::records::register_a_64::RegisterA64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Spill {
  pub inst: u32,
  pub origin: RegisterA64,
  pub slot: i8,
}

impl Default for Spill {
  fn default() -> Self {
    Self {
      inst: 0,
      origin: RegisterA64 { bits: 0 },
      slot: 0,
    }
  }
}

impl Spill {
  pub const INST: u32 = 0;
  pub const SLOT: i8 = 0;
}
