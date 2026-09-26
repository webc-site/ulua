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
        // f64 未实现 Hash，取位表示（对应 C++ DenseHashDefault<double>）。
        // cpp `std::hash<double>` 把 ±0.0 归一同 hash；`PartialEq` 视 0.0 == -0.0，
        // hash 侧必须同样归一，否则 DenseHashMap 定桶与 eq 判等分裂
        let bits = if *value == 0.0 { 0u64 } else { value.to_bits() };
        DenseHashDefault::<u64>::default().hash(&bits)
      }
    }
  }
}
