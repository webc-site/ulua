//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:552-581`

use alloc::vec::Vec;
use core::ptr::{null, null_mut};

use crate::{
  records::{
    serialized_function_scope::SerializedFunctionScope, serialized_generic::SerializedGeneric,
    type_function_runtime::TypeFunctionRuntime,
    type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
  },
  type_aliases::{
    seen_type_packs_type_function_runtime_builder_alt_d::SeenTypePacks,
    seen_types_type_function_runtime_builder_alt_d::SeenTypes,
    type_function_kind::TypeFunctionKind, type_id::TypeId, type_or_pack::TypeOrPack,
    type_pack_id::TypePackId,
  },
};
#[derive(Debug, Clone)]
pub struct TypeFunctionDeserializer {
  pub(crate) state: *mut TypeFunctionRuntimeBuilderState,
  pub(crate) type_function_runtime: *mut TypeFunctionRuntime,
  pub(crate) queue: Vec<(TypeFunctionKind, TypeOrPack)>,
  pub(crate) generic_types: Vec<SerializedGeneric<TypeId>>,
  pub(crate) generic_packs: Vec<SerializedGeneric<TypePackId>>,
  pub(crate) function_scopes: Vec<SerializedFunctionScope>,
  pub(crate) types: SeenTypes,
  pub(crate) packs: SeenTypePacks,
  pub(crate) steps: i32,
}

impl Default for TypeFunctionDeserializer {
  fn default() -> Self {
    Self {
      state: null_mut(),
      type_function_runtime: null_mut(),
      queue: Vec::new(),
      generic_types: Vec::new(),
      generic_packs: Vec::new(),
      function_scopes: Vec::new(),
      types: SeenTypes::new(null()),
      packs: SeenTypePacks::new(null()),
      steps: 0,
    }
  }
}
