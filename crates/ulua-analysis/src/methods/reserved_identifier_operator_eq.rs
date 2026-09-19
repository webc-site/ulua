use crate::records::reserved_identifier::ReservedIdentifier;

impl ReservedIdentifier {
  #[inline]
  pub fn operator_eq(&self, rhs: &ReservedIdentifier) -> bool {
    self.name == rhs.name
  }
}
