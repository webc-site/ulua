use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IllegalRequire {
  pub(crate) module_name: String,
  pub(crate) reason: String,
}

impl IllegalRequire {
  pub const fn new(module_name: String, reason: String) -> Self {
    Self {
      module_name,
      reason,
    }
  }
}

impl IllegalRequire {
  pub fn module_name(&self) -> &str {
    &self.module_name
  }

  pub fn reason(&self) -> &str {
    &self.reason
  }
}
