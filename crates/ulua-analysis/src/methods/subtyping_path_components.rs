//! `Subtyping` 推理路径 `Component` 构造简写：`is_covariant_with`（现行）与
//! `is_covariant_with_deprecated`（旧求解器旗标路径）两侧同款使用，此前两文件
//! 各抄一份，收口于此。

use alloc::string::ToString;

use crate::{
  enums::type_field::TypeField, records::property_type_path::Property as PathProperty,
  type_aliases::component::Component,
};

/// 表属性读写路径分量（cpp `SubtypingReason` 的 `Property{readTy,writeTy}` 形态）。
pub(crate) fn path_property(name: &str, is_read: bool) -> Component {
  Component::Property(PathProperty {
    name: name.to_string(),
    is_read,
  })
}

/// indexer 结果类型分量（cpp `TypeField::IndexResult`）。
pub(crate) fn index_result_component() -> Component {
  Component::TypeField(TypeField::IndexResult)
}
