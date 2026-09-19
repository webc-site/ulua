//! Source: `Analysis/include/Luau/TypeFunctionRuntimeBuilder.h`

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
