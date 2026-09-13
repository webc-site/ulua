use core::mem::zeroed;

use crate::{enums::ir_value_kind::IrValueKind, records::register_x_64::RegisterX64};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrSpillX64 {
  pub inst_idx: u32,
  pub value_kind: IrValueKind,
  pub spill_id: u32,
  /// Spill location can be a stack location or be empty
  /// When it's empty, it means that instruction value can be rematerialized
  pub stack_slot: u8,
  pub original_loc: RegisterX64,
}

impl IrSpillX64 {
  pub const K_NO_STACK_SLOT: u8 = 255;
}

impl Default for IrSpillX64 {
  fn default() -> Self {
    Self {
      inst_idx: 0,
      value_kind: IrValueKind::Unknown,
      spill_id: 0,
      stack_slot: Self::K_NO_STACK_SLOT,
      // noreg is defined as {SizeX64::None, 16} in C++.
      // Since RegisterX64 is a bitfield-like record, we rely on its own internal state.
      // In the absence of a public constructor for RegisterX64 in the provided context,
      // we assume it is initialized to its 'noreg' equivalent.
      original_loc: unsafe { zeroed() },
    }
  }
}
