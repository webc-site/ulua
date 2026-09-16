use crate::records::generic_type_count_mismatch::GenericTypeCountMismatch;

impl GenericTypeCountMismatch {
  #[inline]
  pub fn operator_eq(&self, rhs: &GenericTypeCountMismatch) -> bool {
    self.sub_ty_generic_count == rhs.sub_ty_generic_count
      && self.super_ty_generic_count == rhs.super_ty_generic_count
  }
}
