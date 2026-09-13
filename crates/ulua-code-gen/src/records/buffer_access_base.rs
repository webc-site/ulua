use crate::records::ir_op::IrOp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct BufferAccessBase {
  pub op: IrOp,
  pub scale: i32,
  pub offset: i32,
}
