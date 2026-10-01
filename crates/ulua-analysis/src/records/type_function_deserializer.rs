//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:552-581`

use alloc::vec::Vec;
use core::ptr::null;

use crate::{
  records::{
    arena_handle::Handle, serialized_function_scope::SerializedFunctionScope,
    serialized_generic::SerializedGeneric, type_function_runtime::TypeFunctionRuntime,
    type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
  },
  type_aliases::{
    seen_type_packs_type_function_runtime_builder::DeserializedTypePacks,
    seen_types_type_function_runtime_builder::DeserializerSeenTypes,
    type_function_kind::TypeFunctionKind, type_id::TypeId, type_or_pack::TypeOrPack,
    type_pack_id::TypePackId,
  },
};
#[derive(Debug, Clone)]
pub struct TypeFunctionDeserializer {
  /// 同 `TypeFunctionSerializer`:未装配态用 `Option`,替代 `null_mut()` 哨兵。
  pub(crate) state: Option<Handle<TypeFunctionRuntimeBuilderState>>,
  pub(crate) type_function_runtime: Option<Handle<TypeFunctionRuntime>>,
  pub(crate) queue: Vec<(TypeFunctionKind, TypeOrPack)>,
  pub(crate) generic_types: Vec<SerializedGeneric<TypeId>>,
  pub(crate) generic_packs: Vec<SerializedGeneric<TypePackId>>,
  pub(crate) function_scopes: Vec<SerializedFunctionScope>,
  pub(crate) types: DeserializerSeenTypes,
  pub(crate) packs: DeserializedTypePacks,
  pub(crate) steps: i32,
}

impl Default for TypeFunctionDeserializer {
  fn default() -> Self {
    Self {
      state: None,
      type_function_runtime: None,
      queue: Vec::new(),
      generic_types: Vec::new(),
      generic_packs: Vec::new(),
      function_scopes: Vec::new(),
      types: DeserializerSeenTypes::new(null()),
      packs: DeserializedTypePacks::new(null()),

      steps: 0,
    }
  }
}
