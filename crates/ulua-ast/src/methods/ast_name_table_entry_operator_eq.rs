use core::slice::from_raw_parts;

use crate::records::entry::Entry;

impl Entry {
  pub fn operator_eq(&self, other: &Entry) -> bool {
    if self.length != other.length {
      return false;
    }

    if self.value.value == other.value.value {
      return true;
    }

    unsafe {
      from_raw_parts(self.value.value as *const u8, self.length as usize)
        == from_raw_parts(other.value.value as *const u8, other.length as usize)
    }
  }
}

impl PartialEq for Entry {
  fn eq(&self, other: &Self) -> bool {
    self.operator_eq(other)
  }
}

impl Eq for Entry {}

pub fn ast_name_table_entry_operator_eq(this: &Entry, other: &Entry) -> bool {
  this.operator_eq(other)
}
