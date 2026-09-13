#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Value {
  Suppress,
  DoNotSuppress,
  NormalizationFailed,
}

impl Value {
  pub const SUPPRESS: Value = Value::Suppress;
  pub const DO_NOT_SUPPRESS: Value = Value::DoNotSuppress;
  pub const NORMALIZATION_FAILED: Value = Value::NormalizationFailed;
}
