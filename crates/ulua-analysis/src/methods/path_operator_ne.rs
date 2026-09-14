use crate::records::path::Path;

impl Path {
  pub fn operator_ne(&self, other: &Path) -> bool {
    !self.operator_eq(other)
  }
}
