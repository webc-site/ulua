use alloc::string::String;

use ulua_common::records::variant::Variant2;

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigTableKey(pub Variant2<String, f64>);

impl Default for ConfigTableKey {
  fn default() -> Self {
    Self(Variant2::V0(String::default()))
  }
}

impl From<String> for ConfigTableKey {
  fn from(value: String) -> Self {
    Self(Variant2::V0(value))
  }
}

impl From<&str> for ConfigTableKey {
  fn from(value: &str) -> Self {
    Self::from(String::from(value))
  }
}

impl From<f64> for ConfigTableKey {
  fn from(value: f64) -> Self {
    Self(Variant2::V1(value))
  }
}
