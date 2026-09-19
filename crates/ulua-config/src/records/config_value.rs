use alloc::string::String;

use ulua_common::records::{dense_hash_table::DenseDefault, variant::Variant4};

use crate::records::config_table::ConfigTable;

#[derive(Debug, Clone)]
pub struct ConfigValue(pub Variant4<String, f64, bool, ConfigTable>);

impl ConfigValue {
  pub fn get_string(&self) -> Option<&String> {
    self.0.get_if_0()
  }

  pub fn get_number(&self) -> Option<&f64> {
    self.0.get_if_1()
  }

  pub fn get_bool(&self) -> Option<&bool> {
    self.0.get_if_2()
  }

  pub fn get_table(&self) -> Option<&ConfigTable> {
    self.0.get_if_3()
  }
}

impl Default for ConfigValue {
  fn default() -> Self {
    Self(Variant4::V0(String::new()))
  }
}

impl DenseDefault for ConfigValue {
  fn dense_default() -> Self {
    Self::default()
  }
}

impl From<String> for ConfigValue {
  fn from(value: String) -> Self {
    Self(Variant4::V0(value))
  }
}

impl From<f64> for ConfigValue {
  fn from(value: f64) -> Self {
    Self(Variant4::V1(value))
  }
}

impl From<bool> for ConfigValue {
  fn from(value: bool) -> Self {
    Self(Variant4::V2(value))
  }
}

impl From<ConfigTable> for ConfigValue {
  fn from(value: ConfigTable) -> Self {
    Self(Variant4::V3(value))
  }
}
