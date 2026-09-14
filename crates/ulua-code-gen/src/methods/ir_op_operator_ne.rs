use crate::records::ir_op::IrOp;

impl IrOp {
  #[inline]
  pub const fn ir_op_operator_ne(&self, rhs: IrOp) -> bool {
    self.kind_and_index != rhs.kind_and_index
  }
}

impl PartialEq<IrOp> for &IrOp {
  #[inline]
  fn eq(&self, other: &IrOp) -> bool {
    self.kind_and_index == other.kind_and_index
  }
}
