use crate::records::cannot_compare_unrelated_types::CannotCompareUnrelatedTypes;

impl CannotCompareUnrelatedTypes {
  #[inline]
  pub fn operator_eq(&self, rhs: &CannotCompareUnrelatedTypes) -> bool {
    self.left == rhs.left && self.right == rhs.right && self.op == rhs.op
  }
}
