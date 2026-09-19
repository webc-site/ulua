use crate::records::string_ref::StringRef;

impl StringRef {
  pub fn operator_eq(&self, other: &StringRef) -> bool {
    self == other
  }
}
