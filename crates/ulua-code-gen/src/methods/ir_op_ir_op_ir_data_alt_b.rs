use crate::{enums::ir_op_kind::IrOpKind, records::ir_op::IrOp};

impl IrOp {
  pub fn ir_op_ir_op_kind_u32(kind: IrOpKind, index: u32) -> IrOp {
    IrOp::ir_op_kind_u32(kind, index)
  }
}
