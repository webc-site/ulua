use crate::records::unknown_symbol::UnknownSymbol;

impl UnknownSymbol {
  #[inline]
  pub fn operator_eq(&self, rhs: &UnknownSymbol) -> bool {
    self.name == rhs.name
  }
}
