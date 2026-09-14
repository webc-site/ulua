use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Property {
  pub(crate) name: String,
  pub(crate) is_read: bool,
}

impl Default for Property {
  fn default() -> Self {
    Self {
      name: String::new(),
      is_read: true,
    }
  }
}

impl Property {
  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn is_read(&self) -> bool {
    self.is_read
  }
}
