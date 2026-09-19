use core::{
  hash::{Hash, Hasher},
  ptr::null_mut,
};

use ulua_ast::records::{ast_local::AstLocal, ast_name::AstName};
#[derive(Debug, Clone)]
pub struct Symbol {
  pub(crate) local: *mut AstLocal,
  pub(crate) global: AstName,
}

impl Default for Symbol {
  fn default() -> Self {
    Self {
      local: null_mut(),
      global: AstName::new(),
    }
  }
}

impl Symbol {
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

  #[inline]
  pub fn name(&self) -> &str {
    self.ast_name().as_str().unwrap_or("")
  }

  #[inline]
  pub fn name_str(&self) -> &str {
    self.name()
  }
}

impl PartialEq for Symbol {
  fn eq(&self, rhs: &Self) -> bool {
    if !self.local.is_null() {
      self.local == rhs.local
    } else if !self.global.value.is_null() {
      !rhs.global.value.is_null() && self.global.as_bytes() == rhs.global.as_bytes()
    } else {
      rhs.local.is_null() && rhs.global.value.is_null()
    }
  }
}

impl Eq for Symbol {}

impl Hash for Symbol {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.local.hash(state);

    if !self.global.value.is_null() {
      self.global.as_bytes().hash(state);
    }
  }
}

// SAFETY: `local` 裸指针仅作身份比较（eq/hash），从不解引用；
// `global` 的 AstName 指向全局字符串驻留表，跨线程只读共享安全。
unsafe impl Send for Symbol {}
unsafe impl Sync for Symbol {}
