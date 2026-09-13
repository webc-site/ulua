use crate::records::{pack_slice::PackSlice, path_hash::PathHash};

impl PathHash {
  pub fn operator_call_5(&self, slice: &PackSlice) -> usize {
    slice.start_index
  }
}
