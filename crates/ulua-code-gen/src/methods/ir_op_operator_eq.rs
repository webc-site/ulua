use crate::records::ir_op::IrOp;

impl IrOp {
  #[inline]
  pub fn ir_op_operator_eq(&self, rhs: IrOp) -> bool {
    self.kind() == rhs.kind() && self.index() == rhs.index()
  }
}
