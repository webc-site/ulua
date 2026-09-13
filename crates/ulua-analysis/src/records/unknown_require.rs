use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnknownRequire {
  pub(crate) module_path: String,
}

impl UnknownRequire {
  pub const fn new(module_path: String) -> Self {
    Self { module_path }
  }
}

impl UnknownRequire {
  pub fn module_path(&self) -> &str {
    &self.module_path
  }
}
