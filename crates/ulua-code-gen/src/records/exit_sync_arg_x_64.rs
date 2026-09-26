use crate::records::{register_x_64::RegisterX64, value_restore_location::ValueRestoreLocation};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ExitSyncArgX64 {
  pub inst_idx: u32,
  pub reg: RegisterX64,
  pub stack_slot: u8,
  pub original_reg: RegisterX64,
  pub restore_location: ValueRestoreLocation,
}

impl Default for ExitSyncArgX64 {
  fn default() -> Self {
    Self {
      inst_idx: 0,
      // cpp `RegisterX64 reg = noreg`：noreg 是 {None, 16}，而非 zeroed 的 {None, 0}(RIP)
      reg: RegisterX64::NOREG,
      stack_slot: 255,
      original_reg: RegisterX64::NOREG,
      restore_location: ValueRestoreLocation::default(),
    }
  }
}
