use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  config_table_key::ConfigTableKey, config_value::ConfigValue,
  variant_hash_default::VariantHashDefault,
};

#[derive(Debug, Clone)]
pub struct ConfigTable {
  pub map: DenseHashMap<ConfigTableKey, ConfigValue, VariantHashDefault>,
}

impl ConfigTable {
  pub fn new() -> Self {
    Self {
      map: DenseHashMap::new(ConfigTableKey::default()),
    }
  }

  pub fn get_or_insert(&mut self, key: ConfigTableKey) -> &mut ConfigValue {
    self.map.get_or_insert(key)
  }

  pub fn insert(&mut self, key: ConfigTableKey, value: ConfigValue) {
    *self.get_or_insert(key) = value;
  }

  pub fn find(&self, key: &ConfigTableKey) -> Option<&ConfigValue> {
    self.map.find(key)
  }

  pub fn find_str(&self, key: &str) -> Option<&ConfigValue> {
    self.find(&ConfigTableKey::from(key))
  }

  pub fn contains_str(&self, key: &str) -> bool {
    self.map.contains(&ConfigTableKey::from(key))
  }

  pub fn size(&self) -> usize {
    self.map.size()
  }

  pub fn iter(&self) -> impl Iterator<Item = (&ConfigTableKey, &ConfigValue)> {
    self.map.iter()
  }
}

impl Default for ConfigTable {
  fn default() -> Self {
    Self::new()
  }
}
