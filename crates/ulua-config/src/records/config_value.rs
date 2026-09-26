use alloc::string::String;

use ulua_common::records::dense_hash_table::DenseDefault;

use crate::records::config_table::ConfigTable;

/// 对应 C++ `ConfigValue = Variant<std::string, double, bool, ConfigTable>`。
/// r7-variant4：由位置式 `Variant4` + `V0..V3`/`get_if_0..3` 改写为直接语义
/// enum（review.md §3/§7 消灭位置式 C 风格命名；消费面已核实收敛于 ulua-config
/// ±测试，其余 `VariantN` 调用方在 ast/analysis/compiler，不波及本 enum）。
#[derive(Debug, Clone)]
pub enum ConfigValue {
  String(String),
  F64(f64),
  Bool(bool),
  Table(ConfigTable),
}

impl ConfigValue {
  pub fn get_string(&self) -> Option<&String> {
    match self {
      Self::String(value) => Some(value),
      _ => None,
    }
  }

  pub fn get_bool(&self) -> Option<&bool> {
    match self {
      Self::Bool(value) => Some(value),
      _ => None,
    }
  }

  pub fn get_table(&self) -> Option<&ConfigTable> {
    match self {
      Self::Table(value) => Some(value),
      _ => None,
    }
  }
}

impl Default for ConfigValue {
  fn default() -> Self {
    Self::String(String::new())
  }
}

impl DenseDefault for ConfigValue {
  fn dense_default() -> Self {
    Self::default()
  }
}

impl From<String> for ConfigValue {
  fn from(value: String) -> Self {
    Self::String(value)
  }
}

impl From<f64> for ConfigValue {
  fn from(value: f64) -> Self {
    Self::F64(value)
  }
}

impl From<bool> for ConfigValue {
  fn from(value: bool) -> Self {
    Self::Bool(value)
  }
}

impl From<ConfigTable> for ConfigValue {
  fn from(value: ConfigTable) -> Self {
    Self::Table(value)
  }
}
