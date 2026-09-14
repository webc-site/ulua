use crate::enums::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ErrorSuppression {
  pub(crate) value: Value,
}

impl ErrorSuppression {
  pub const fn new() -> Self {
    Self {
      value: Value::Suppress,
    }
  }

  pub const fn from_value(enum_value: Value) -> Self {
    Self { value: enum_value }
  }

  pub fn or_else(&self, other: &Self) -> Self {
    match self.value {
      Value::DoNotSuppress => *other,
      _ => *self,
    }
  }
}

impl Default for ErrorSuppression {
  fn default() -> Self {
    Self {
      value: Value::Suppress,
    }
  }
}

impl From<ErrorSuppression> for Value {
  fn from(suppression: ErrorSuppression) -> Self {
    suppression.value
  }
}

impl From<Value> for ErrorSuppression {
  fn from(value: Value) -> Self {
    Self { value }
  }
}

unsafe impl Send for ErrorSuppression {}
unsafe impl Sync for ErrorSuppression {}
