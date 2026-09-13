use alloc::string::String;

use ulua_common::{
  records::{dense_hash_table::DenseHasher, variant::Variant2},
  type_aliases::dense_hash_default::DenseHashDefault,
};

use crate::records::{config_table_key::ConfigTableKey, variant_hash_default::VariantHashDefault};

impl DenseHasher<ConfigTableKey> for VariantHashDefault {
  fn hash(&self, key: &ConfigTableKey) -> usize {
    match &key.0 {
      Variant2::V0(value) => DenseHashDefault::<String>::default().hash(value),
      Variant2::V1(value) => {
        // f64 未实现 Hash，取位表示（对应 C++ DenseHashDefault<double>）
        DenseHashDefault::<u64>::default().hash(&value.to_bits())
      }
    }
  }
}
