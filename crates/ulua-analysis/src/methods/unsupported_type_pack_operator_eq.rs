use crate::records::unsupported_type_pack::UnsupportedTypePack;

impl UnsupportedTypePack {
  #[inline]
  pub fn operator_eq(&self, rhs: &UnsupportedTypePack) -> bool {
    self.pack == rhs.pack
  }
}
