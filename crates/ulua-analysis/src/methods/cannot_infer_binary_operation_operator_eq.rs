use crate::records::cannot_infer_binary_operation::CannotInferBinaryOperation;

impl CannotInferBinaryOperation {
  #[inline]
  pub fn operator_eq(&self, rhs: &CannotInferBinaryOperation) -> bool {
    self.op == rhs.op
      && self.suggested_to_annotate == rhs.suggested_to_annotate
      && self.kind == rhs.kind
  }
}
