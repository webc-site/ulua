use crate::{enums::ir_value_kind::IrValueKind, records::register_x_64::RegisterX64};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrSpillX64 {
  pub inst_idx: u32,
  pub value_kind: IrValueKind,
  pub spill_id: u32,
  /// spill 位置可以是栈槽或为空
  /// 为空表示指令的值可 rematerialize
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
      // cpp `RegisterX64 originalLoc = noreg`：noreg = {None, 16}，
      // 旧实现 zeroed 得到 {None, 0}(RIP)，与 cpp 默认值不符，已修正
      original_loc: RegisterX64::NOREG,
    }
  }
}
