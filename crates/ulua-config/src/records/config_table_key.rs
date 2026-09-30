use alloc::string::String;

/// 对应 C++ `ConfigTableKey = Variant<std::string, double>`。
/// r7-variant4：由位置式 `Variant2` + `V0`/`V1` 改写为直接语义 enum
/// （review.md §3/§7；消费面收敛于 ulua-config ±测试，见 config_value.rs 注释）。
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigTableKey {
  String(String),
  F64(f64),
}

impl Default for ConfigTableKey {
  fn default() -> Self {
    Self::String(String::default())
  }
}

impl From<String> for ConfigTableKey {
  fn from(value: String) -> Self {
    Self::String(value)
  }
}

impl From<&str> for ConfigTableKey {
  fn from(value: &str) -> Self {
    Self::from(String::from(value))
  }
}

impl From<f64> for ConfigTableKey {
  fn from(value: f64) -> Self {
    Self::F64(value)
  }
}
