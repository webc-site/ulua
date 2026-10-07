//! Source: `Analysis/src/TypeFunctionRuntime.cpp`

use alloc::vec::Vec;

use crate::{
  records::{arena_handle::Handle, type_function_runtime::TypeFunctionRuntime},
  type_aliases::{
    seen_type_packs_type_function_runtime::SeenTypePacks,
    seen_types_type_function_runtime::SeenTypes, type_function_kind::TypeFunctionKind,
  },
};

#[derive(Debug, Clone)]
pub struct TypeFunctionCloner {
  /// cpp `TypeFunctionCloner(TypeFunctionRuntime& runtime)`:会话单例的别名句柄,
  /// 非空由 [`Handle`] 编码,不再以裸指针流转(review.md §2)。
  pub(crate) type_function_runtime: Handle<TypeFunctionRuntime>,
  pub(crate) queue: Vec<(TypeFunctionKind, TypeFunctionKind)>,
  pub(crate) types: SeenTypes,
  pub(crate) packs: SeenTypePacks,
  pub(crate) steps: i32,
}
