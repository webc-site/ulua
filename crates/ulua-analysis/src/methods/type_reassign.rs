use crate::records::r#type::Type;

impl Type {
  pub fn reassign(&mut self, rhs: &Type) {
    self.ty = rhs.ty.clone();
    self.documentation_symbol = rhs.documentation_symbol.clone();
  }
}
