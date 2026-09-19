use ulua_common::functions::hash_range::hash_range_bytes;

use crate::records::{string_ref::StringRef, string_ref_hash::StringRefHash};

impl StringRefHash {
  pub fn operator_call(&self, v: &StringRef) -> usize {
    hash_range_bytes(v.as_bytes())
  }
}
