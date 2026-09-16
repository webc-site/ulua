//! Generated skeleton item.
//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/TypeFunctionRuntimeBuilder.h:20:type_function_runtime_builder_state`
//! Source: `Analysis/include/Luau/TypeFunctionRuntimeBuilder.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/TypeFunctionRuntimeBuilder.h
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/TypeFunctionError.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/TypeFunctionRuntimeBuilder.h
//!   - type_ref <- record TypeFunctionRuntime (Analysis/include/Luau/TypeFunctionRuntime.h)
//!   - type_ref <- function isSubtypeOf (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- record TypeFunctionSerializer (Analysis/src/TypeFunctionRuntimeBuilder.cpp)
//!   - type_ref <- method TypeFunctionSerializer::TypeFunctionSerializer (Analysis/src/TypeFunctionRuntimeBuilder.cpp)
//!   - type_ref <- record TypeFunctionDeserializer (Analysis/src/TypeFunctionRuntimeBuilder.cpp)
//!   - type_ref <- method TypeFunctionDeserializer::TypeFunctionDeserializer (Analysis/src/TypeFunctionRuntimeBuilder.cpp)
//!   - type_ref <- function serialize (Analysis/src/TypeFunctionRuntimeBuilder.cpp)
//!   - type_ref <- function serialize (Analysis/src/TypeFunctionRuntimeBuilder.cpp)
//!   - type_ref <- function deserialize (Analysis/src/TypeFunctionRuntimeBuilder.cpp)
//!   - type_ref <- function deserialize (Analysis/src/TypeFunctionRuntimeBuilder.cpp)
//!   - type_ref <- function evaluateTypeAliasCall (Analysis/src/UserDefinedTypeFunction.cpp)
//!   - type_ref <- function userDefinedTypeFunction (Analysis/src/UserDefinedTypeFunction.cpp)
//!   - type_ref <- method TypeFunctionRuntimeBuilderState::TypeFunctionRuntimeBuilderState (Analysis/include/Luau/TypeFunctionRuntimeBuilder.h)
//! - outgoing:
//!   - type_ref -> method TypeFunctionRuntimeBuilderState::TypeFunctionRuntimeBuilderState (Analysis/include/Luau/TypeFunctionRuntimeBuilder.h)
//!   - type_ref -> record TypeFunctionContext (Analysis/include/Luau/TypeFunction.h)
//!   - type_ref -> record TypeFunctionError (Analysis/include/Luau/TypeFunctionError.h)
//!   - translates_to -> rust_item TypeFunctionRuntimeBuilderState

extern crate alloc;

use alloc::{string::String, vec::Vec};

use crate::records::{
  type_function_context::TypeFunctionContext, type_function_error::TypeFunctionError,
};

#[derive(Debug)]
pub struct TypeFunctionRuntimeBuilderState {
  pub ctx: *mut TypeFunctionContext,
  // List of errors that occur during serialization/deserialization
  // At every iteration, if this list is non-empty, the process halts.
  pub errors_deprecated: Vec<String>,
  pub errors: Vec<TypeFunctionError>,
}

impl TypeFunctionRuntimeBuilderState {
  pub fn new(ctx: *mut TypeFunctionContext) -> Self {
    Self {
      ctx,
      errors_deprecated: Vec::new(),
      errors: Vec::new(),
    }
  }
}
