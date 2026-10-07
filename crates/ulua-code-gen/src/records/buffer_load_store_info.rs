use crate::{enums::ir_cmd::IrCmd, records::ir_op::IrOp};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct BufferLoadStoreInfo {
  pub load_cmd: IrCmd,
  pub access_size: u8,
  pub tag: u8,
  pub from_store: bool,
  pub address: IrOp,
  pub value: IrOp,
  pub offset: i32,
}

impl Default for BufferLoadStoreInfo {
  fn default() -> Self {
    Self {
      load_cmd: IrCmd::NOP,
      access_size: 0,
      tag: 0,
      from_store: false,
      address: IrOp { kind_and_index: 0 },
      value: IrOp { kind_and_index: 0 },
      offset: 0,
    }
  }
}
