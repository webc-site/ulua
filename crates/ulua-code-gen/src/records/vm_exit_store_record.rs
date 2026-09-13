use core::mem::zeroed;

use crate::records::ir_inst::IrInst;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct VmExitStoreRecord {
  pub inst_idx: u32,
  pub backup: IrInst,
}

impl VmExitStoreRecord {
  pub const INST_IDX: u32 = 0xffffffff;
}

impl Default for VmExitStoreRecord {
  fn default() -> Self {
    Self {
      inst_idx: 0xffffffff,
      backup: unsafe { zeroed() },
    }
  }
}
