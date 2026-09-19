use crate::records::builtin::Builtin;

impl Builtin {
  pub fn is_method(&self, table: &str, name: &str) -> bool {
    if self.object.is_null() || self.method.is_null() {
      return false;
    }
    self.object.as_bytes() == table.as_bytes() && self.method.as_bytes() == name.as_bytes()
  }
}
