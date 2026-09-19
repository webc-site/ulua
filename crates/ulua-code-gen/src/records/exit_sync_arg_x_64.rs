use core::mem::zeroed;

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
      reg: unsafe { zeroed() },
      stack_slot: 255,
      original_reg: unsafe { zeroed() },
      restore_location: unsafe { zeroed() },
    }
  }
}
