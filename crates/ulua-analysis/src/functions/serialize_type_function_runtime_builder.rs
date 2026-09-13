//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:1106-1109`
//!
//! ```cpp
//! TypeFunctionTypeId serialize(TypeId ty, TypeFunctionRuntimeBuilderState* state)
//! {
//!     return TypeFunctionSerializer(state).serialize(ty);
//! }
//! ```
use alloc::vec::Vec;
use core::ptr::{null, null_mut};

use crate::{
  records::{
    type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
    type_function_serializer::TypeFunctionSerializer,
  },
  type_aliases::{
    seen_type_packs_type_function_runtime_builder::SeenTypePacks,
    seen_types_type_function_runtime_builder::SeenTypes, type_function_type_id::TypeFunctionTypeId,
    type_id::TypeId,
  },
};
pub(crate) fn serialize_type_id_type_function_runtime_builder_state(
  ty: TypeId,
  state: *mut TypeFunctionRuntimeBuilderState,
) -> TypeFunctionTypeId {
  // C++ constructs a temporary `TypeFunctionSerializer(state)` and immediately calls
  // `.serialize(ty)`. The `TypeFunctionSerializer(state)` constructor is `type_function_serializer`.
  let mut serializer = TypeFunctionSerializer {
    state: null_mut(),
    type_function_runtime: null_mut(),
    queue: Vec::new(),
    types: SeenTypes::new(null()),
    packs: SeenTypePacks::new(null()),
    steps: 0,
  };
  unsafe { serializer.type_function_serializer(state) };
  serializer.serialize_type_id(ty)
}
