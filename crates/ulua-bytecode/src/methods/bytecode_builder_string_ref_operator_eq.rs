use core::ptr;

use crate::records::string_ref::StringRef;

impl StringRef {
  pub fn operator_eq(&self, other: &StringRef) -> bool {
    if !self.data.is_null() && !other.data.is_null() {
      if self.length != other.length {
        return false;
      }
      if self.length == 0 {
        return true;
      }
      ptr::eq(self.data, other.data) || self.as_bytes() == other.as_bytes()
    } else {
      self.data == other.data
    }
  }
}
