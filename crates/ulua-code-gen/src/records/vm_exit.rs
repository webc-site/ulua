extern crate alloc;
use alloc::vec::Vec;

use ulua_common::records::small_vector::SmallVector;

use crate::{
  enums::{ir::IrValueKind, ir_cmd::IrCmd},
  records::{
    ir_inst::IrInst, ir_op::IrOp, label::Label, register_a_64::RegisterA64,
    register_x_64::RegisterX64,
  },
};

/// cpp `IrLoweringA64.h:83` / `IrLoweringX64.h:87` 各自声明的同名嵌套结构；
/// 两处字段完全一致，故合并为单一定义供两个后端共用。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[derive(Default)]
pub struct ExitHandler {
  pub self_: Label,
  pub pcpos: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct InterruptHandler {
  pub self_: Label,
  pub pcpos: u32,
  pub next: Label,
}

/// gate 入口机器码的标签组（原 `entry_locations_code_gen_a_64` / `entry_locations_code_gen_x_64`
/// 两份逐字同构的定义收口为此单一来源）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct EntryLocations {
  pub start: Label,
  pub prologue_end: Label,
  pub epilogue_start: Label,
}

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
      // IrOp{0}、Unknown、NOP 均为判别值 0，与 zeroed 位等价，直接复用安全 Default
      restore_location: ValueRestoreLocation::default(),
    }
  }
}

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

#[derive(Debug, Clone)]
#[repr(C)]
pub struct VmExitStoreInfo {
  pub reg: u8,
  pub stores: SmallVector<VmExitStoreRecord, 2>,
}

impl Default for VmExitStoreInfo {
  fn default() -> Self {
    Self {
      reg: 0,
      stores: SmallVector::new(),
    }
  }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct VmExitStoreRecord {
  pub inst_idx: u32,
  pub backup: IrInst,
}

impl Default for VmExitStoreRecord {
  fn default() -> Self {
    Self {
      inst_idx: 0xffffffff,
      // IrInst 含 SmallVector 与 noreg 寄存器，zeroed 会造出非法堆状态；
      // 安全 Default 即 cpp 默认（NOP、空操作数、noreg）
      backup: IrInst::default(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct VmExitSyncInfo {
  pub reg_stores: Vec<VmExitStoreInfo>,
  pub block: IrOp,
  pub vm_exit: IrOp,
  pub arg_ops: SmallVector<IrOp, 2>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ValueRestoreLocation {
  pub op: IrOp,
  pub kind: IrValueKind,
  pub conversion_cmd: IrCmd,
  pub lazy: bool,
}

impl Default for ValueRestoreLocation {
  fn default() -> Self {
    Self {
      op: IrOp { kind_and_index: 0 },
      kind: IrValueKind::Unknown,
      conversion_cmd: IrCmd::NOP,
      lazy: false,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct StoreLocationHint {
  pub op: IrOp,
  pub inst_idx: u32,
  pub kind: IrValueKind,
}
