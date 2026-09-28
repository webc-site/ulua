//! C++ `Luau::TypeAnnotationRequired`（`Error.h:622-627`）。
//!
//! 顶层/类成员函数缺少源码标注时的提示诊断；`inferred_ty` 为推导出的函数类型，
//! 报错文案会尝试将其作为建议标注输出（`Error.cpp:1035-1044`）。

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeAnnotationRequired {
  pub inferred_ty: TypeId,
}

impl TypeAnnotationRequired {
  pub const fn new(inferred_ty: TypeId) -> Self {
    Self { inferred_ty }
  }
}
