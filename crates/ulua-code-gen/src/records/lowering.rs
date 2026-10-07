use crate::records::{ir_op::IrOp, register_a_64::RegisterA64};

#[derive(Debug, Clone, Copy)]
pub struct BuiltinArgs {
  pub ra: i32,
  pub arg: i32,
  pub args: IrOp,
  pub arg3: IrOp,
  pub nparams: i32,
  pub nresults: i32,
  pub pcpos: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct BufferAccessBase {
  pub op: IrOp,
  pub scale: i32,
  pub offset: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ArrayValueEntry {
  pub pointer: u32,
  pub offset: IrOp,
  pub value: u32,
}

impl Default for ArrayValueEntry {
  fn default() -> Self {
    Self {
      pointer: 0,
      offset: IrOp { kind_and_index: 0 },
      value: 0,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct LoopInfo {
  pub step: IrOp,
  pub startpc: i32,
}

impl Default for LoopInfo {
  fn default() -> Self {
    Self {
      step: IrOp { kind_and_index: 0 },
      startpc: 0,
    }
  }
}

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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RegisterInfo {
  pub tag: u8,
  pub value: IrOp,
  pub version: u32,
}

impl Default for RegisterInfo {
  fn default() -> Self {
    Self {
      tag: 0xff,
      value: IrOp::default(),
      version: 0,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct NodeSlotState {
  pub pointer: u32,
  pub known_to_not_be_nil: bool,
}
