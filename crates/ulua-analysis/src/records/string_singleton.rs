use alloc::string::String;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct StringSingleton {
  pub value: String,
}

impl StringSingleton {
  pub const fn new(value: String) -> Self {
    Self { value }
  }
}

unsafe impl Send for StringSingleton {}
unsafe impl Sync for StringSingleton {}
