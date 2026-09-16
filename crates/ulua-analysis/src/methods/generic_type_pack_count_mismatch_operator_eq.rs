use crate::records::generic_type_pack_count_mismatch::GenericTypePackCountMismatch;

impl GenericTypePackCountMismatch {
  #[inline]
  pub fn operator_eq(&self, rhs: &GenericTypePackCountMismatch) -> bool {
    self.sub_ty_generic_pack_count == rhs.sub_ty_generic_pack_count
      && self.super_ty_generic_pack_count == rhs.super_ty_generic_pack_count
  }
}
