use core::{ffi::c_char, ptr::null_mut};

use ulua_ast::records::{ast_local::AstLocal, ast_name::AstName};
use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_table::DenseDefault};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Symbol {
  pub local: *mut AstLocal,
  pub global: AstName,
}

impl Symbol {
  pub fn new() -> Self {
    Self {
      local: null_mut(),
      global: AstName::new(),
    }
  }

  pub fn from_local(local: *mut AstLocal) -> Self {
    Self {
      local,
      global: AstName::new(),
    }
  }

  pub fn from_global(global: AstName) -> Self {
    Self {
      local: null_mut(),
      global,
    }
  }

  pub fn ast_name(&self) -> AstName {
    if !self.local.is_null() {
      unsafe { (*self.local).name }
    } else {
      LUAU_ASSERT!(!self.global.value.is_null());
      self.global
    }
  }

  pub fn c_str(&self) -> *const c_char {
    if !self.local.is_null() {
      unsafe { (*self.local).name.value }
    } else {
      LUAU_ASSERT!(!self.global.value.is_null());
      self.global.value
    }
  }
}

impl Default for Symbol {
  fn default() -> Self {
    Self::new()
  }
}

impl DenseDefault for Symbol {
  fn dense_default() -> Self {
    Self::default()
  }
}
