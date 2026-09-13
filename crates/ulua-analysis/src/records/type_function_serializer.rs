use alloc::vec::Vec;

use crate::{
  records::{
    type_function_runtime::TypeFunctionRuntime,
    type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
  },
  type_aliases::{
    seen_type_packs_type_function_runtime_builder::SeenTypePacks,
    seen_types_type_function_runtime_builder::SeenTypes, type_function_kind::TypeFunctionKind,
    type_or_pack::TypeOrPack,
  },
};
#[derive(Debug, Clone)]
pub struct TypeFunctionSerializer {
  pub(crate) state: *mut TypeFunctionRuntimeBuilderState,
  pub(crate) type_function_runtime: *mut TypeFunctionRuntime,
  pub(crate) queue: Vec<(TypeOrPack, TypeFunctionKind)>,
  pub(crate) types: SeenTypes,
  pub(crate) packs: SeenTypePacks,
  pub(crate) steps: i32,
}
