use crate::{enums::ir_op_kind::IrOpKind, records::ir_op::IrOp};

impl IrOp {
  pub fn new() -> IrOp {
    IrOp::ir_op_kind_u32(IrOpKind::None, 0)
  }

  pub fn ir_op_kind_u32(kind: IrOpKind, index: u32) -> IrOp {
    IrOp {
      kind_and_index: (kind as u32) | (index << IrOp::INDEX_SHIFT),
    }
  }
}

pub fn new() -> IrOp {
  IrOp::new()
}
