use crate::records::pack_slice::PackSlice;

impl PackSlice {
  pub fn operator_eq(&self, other: &PackSlice) -> bool {
    self.start_index == other.start_index
  }
}
