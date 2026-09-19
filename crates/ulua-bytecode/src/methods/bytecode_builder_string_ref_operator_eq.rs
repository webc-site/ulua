use crate::records::string_ref::StringRef;

impl StringRef<'_> {
  pub fn operator_eq(&self, other: &Self) -> bool {
    self == other
  }
}
