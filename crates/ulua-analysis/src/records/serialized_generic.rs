use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SerializedGeneric<T> {
  pub(crate) is_named: bool,
  pub(crate) name: String,
  pub(crate) r#type: T,
}

impl<T: Default> Default for SerializedGeneric<T> {
  fn default() -> Self {
    Self {
      is_named: false,
      name: String::new(),
      r#type: T::default(),
    }
  }
}

impl<T> SerializedGeneric<T> {
  pub fn is_named(&self) -> bool {
    self.is_named
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn r#type(&self) -> &T {
    &self.r#type
  }
}
