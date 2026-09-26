//! Normalizer 四个外域类型合并方法共用的成对删除原语。
use crate::records::{normalized_extern_type::NormalizedExternType, type_ids::TypeIds};

impl NormalizedExternType {
  /// 将 `ordering[idx]` 处的外域类型与其取反集合成对移除，维持
  /// ordering/extern_types 的成对登记不变式；返回被移除的取反集合，
  /// `extern_types` 无该条目时返回空集（对齐 cpp `erase` 的宽容语义）。
  pub fn remove_cluster_at(&mut self, idx: usize) -> TypeIds {
    let ty = self.ordering.remove(idx);
    self.extern_types.remove(&ty).unwrap_or_default()
  }
}
