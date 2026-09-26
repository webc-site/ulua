use crate::records::ir_op::IrOp;

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
