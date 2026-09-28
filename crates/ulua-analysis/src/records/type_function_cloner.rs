//! Source: `Analysis/src/TypeFunctionRuntime.cpp`

use alloc::vec::Vec;

use crate::{
  records::type_function_runtime::TypeFunctionRuntime,
  type_aliases::{
    seen_type_packs_type_function_runtime::SeenTypePacks,
    seen_types_type_function_runtime::SeenTypes, type_function_kind::TypeFunctionKind,
  },
};

#[derive(Debug, Clone)]
pub struct TypeFunctionCloner {
  pub(crate) type_function_runtime: *mut TypeFunctionRuntime,
  pub(crate) queue: Vec<(TypeFunctionKind, TypeFunctionKind)>,
  pub(crate) types: SeenTypes,
  pub(crate) packs: SeenTypePacks,
  pub(crate) steps: i32,
}
