//! Normalizer 四个外域类型（NormalizedExternType）合并方法共用的 negations 访问入口。
//!
//! 成对登记不变式：`ordering` 是 `extern_types` 键的拓扑面列表，二者由 `push_pair`
//! 成对写入，按 `ordering` 元素查表恒命中（对应 C++ `externTypes.at(ty)`）。
use crate::{
  records::{normalized_extern_type::NormalizedExternType, type_ids::TypeIds},
  type_aliases::type_id::TypeId,
};

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
