use crate::records::unexpected_type_in_subtyping::UnexpectedTypeInSubtyping;

impl UnexpectedTypeInSubtyping {
  #[inline]
  pub fn operator_eq(&self, rhs: &UnexpectedTypeInSubtyping) -> bool {
    self.ty == rhs.ty
  }
}
