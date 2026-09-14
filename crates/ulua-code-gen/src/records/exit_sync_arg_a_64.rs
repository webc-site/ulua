use core::mem::zeroed;

use crate::records::{register_a_64::RegisterA64, value_restore_location::ValueRestoreLocation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ExitSyncArgA64 {
  pub inst_idx: u32,
  pub reg: RegisterA64,
  pub slot: i8,
  pub original_reg: RegisterA64,
  pub restore_location: ValueRestoreLocation,
}

impl Default for ExitSyncArgA64 {
  fn default() -> Self {
    Self {
      inst_idx: 0,
      reg: RegisterA64 { bits: 0 },
      slot: -1,
      original_reg: RegisterA64 { bits: 0 },
      restore_location: ValueRestoreLocation {
        op: unsafe { zeroed() },
        kind: unsafe { zeroed() },
        conversion_cmd: unsafe { zeroed() },
        lazy: false,
      },
    }
  }
}
