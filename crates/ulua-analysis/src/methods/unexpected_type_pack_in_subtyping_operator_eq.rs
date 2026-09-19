use crate::records::unexpected_type_pack_in_subtyping::UnexpectedTypePackInSubtyping;

impl UnexpectedTypePackInSubtyping {
  #[inline]
  pub fn operator_eq(&self, rhs: &UnexpectedTypePackInSubtyping) -> bool {
    self.tp == rhs.tp
  }
}
