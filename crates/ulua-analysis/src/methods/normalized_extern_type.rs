//! `normalized_extern_type` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{normalized_extern_type::NormalizedExternType, type_ids::TypeIds},
  type_aliases::type_id::TypeId,
};

impl NormalizedExternType {
  pub fn is_never(&self) -> bool {
    self.extern_types.is_empty()
  }
}

// Normalizer 四个外域类型（NormalizedExternType）合并方法共用的 negations 访问入口。
//
// 成对登记不变式：`ordering` 是 `extern_types` 键的拓扑面列表，二者由 `push_pair`
// 成对写入，按 `ordering` 元素查表恒命中（对应 C++ `externTypes.at(ty)`）。

impl NormalizedExternType {
  /// 按 `ordering` 已登记的外域类型读取其取反集合。
  pub fn negations(&self, ty: TypeId) -> &TypeIds {
    self
      .extern_types
      .get(&ty)
      .expect("ordering 元素必已成对登记于 extern_types")
  }

  /// 按 `ordering` 已登记的外域类型原地改写其取反集合（对齐 cpp 的
  /// `TypeIds& hereNegations = heres.externTypes.at(hereTy)`：必须原地修改，
  /// 克隆回写会丢失 erase）。
  pub fn negations_mut(&mut self, ty: TypeId) -> &mut TypeIds {
    self
      .extern_types
      .get_mut(&ty)
      .expect("ordering 元素必已成对登记于 extern_types")
  }
}

impl NormalizedExternType {
  pub fn push_pair(&mut self, ty: TypeId, negations: TypeIds) {
    let result = self.extern_types.insert(ty, negations);

    if result.is_none() {
      self.ordering.push(ty);
    }

    LUAU_ASSERT!(self.ordering.len() == self.extern_types.len());
  }
}

// Normalizer 四个外域类型合并方法共用的成对删除原语。

impl NormalizedExternType {
  /// 将 `ordering[idx]` 处的外域类型与其取反集合成对移除，维持
  /// ordering/extern_types 的成对登记不变式；返回被移除的取反集合，
  /// `extern_types` 无该条目时返回空集（对齐 cpp `erase` 的宽容语义）。
  pub fn remove_cluster_at(&mut self, idx: usize) -> TypeIds {
    let ty = self.ordering.remove(idx);
    self.extern_types.remove(&ty).unwrap_or_default()
  }
}

impl NormalizedExternType {
  pub fn reset_to_never(&mut self) {
    self.ordering.clear();
    self.extern_types.clear();
    self.shape_extensions.clear();
  }
}
