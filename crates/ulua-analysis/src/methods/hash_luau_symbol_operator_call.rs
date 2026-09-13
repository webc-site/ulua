use core::{
  ffi::CStr,
  hash::{Hash, Hasher},
};
use std::collections::hash_map::DefaultHasher;

use crate::records::symbol::Symbol;
impl Symbol {
  pub fn hash_luau_symbol_operator_call(&self) -> usize {
    let mut hasher = DefaultHasher::new();

    // std::hash<const Luau::AstLocal*>()(s.local)
    self.local.hash(&mut hasher);
    let mut h = hasher.finish() as usize;

    // (s.global.value ? std::hash<std::string_view>()(s.global.value) : 0)
    if !self.global.value.is_null() {
      let mut name_hasher = DefaultHasher::new();
      let s = unsafe { CStr::from_ptr(self.global.value).to_bytes() };
      s.hash(&mut name_hasher);
      h ^= name_hasher.finish() as usize;
    }

    h
  }
}
