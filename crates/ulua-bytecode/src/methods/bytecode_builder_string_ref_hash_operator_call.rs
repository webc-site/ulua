use core::ffi::c_char;

use ulua_common::functions::hash_range::hash_range;

use crate::records::{string_ref::StringRef, string_ref_hash::StringRefHash};

impl StringRefHash {
  pub fn operator_call(&self, v: &StringRef) -> usize {
    // SAFETY：hash_range 要求 data 指针在 length 字节内可读；
    // StringRef 指向有效缓冲区，或为 null（hash_range 自行处理）。
    unsafe { hash_range(v.data as *const c_char, v.length) }
  }
}
