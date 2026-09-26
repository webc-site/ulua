use alloc::string::String;
use core::ptr::null;

use crate::{
  enums::context_error::Context, records::type_mismatch::TypeMismatch,
  type_aliases::type_id::TypeId,
};

impl TypeMismatch {
  pub fn new() -> Self {
    Self {
      wanted_type: null(),
      given_type: null(),
      reason: String::new(),
      error: None,
      context: Context::CovariantContext,
    }
  }

  // C++ `TypeMismatch::TypeMismatch(TypeId wantedType, TypeId givenType)`
  // (Error.cpp:1026): sets the two types and defaults reason/error/context.
  pub fn from_wanted_given(wanted_type: TypeId, given_type: TypeId) -> Self {
    Self {
      wanted_type,
      given_type,
      reason: String::new(),
      error: None,
      context: Context::CovariantContext,
    }
  }
}

impl Default for TypeMismatch {
  fn default() -> Self {
    Self::new()
  }
}

impl TypeMismatch {
  // C++ `TypeMismatch::TypeMismatch(TypeId wantedType, TypeId givenType, std::string reason)`
  // (Error.cpp:1032).
  pub fn from_wanted_given_reason(wanted_type: TypeId, given_type: TypeId, reason: String) -> Self {
    Self {
      wanted_type,
      given_type,
      reason,
      error: None,
      context: Context::CovariantContext,
    }
  }

  // C++ `TypeMismatch::TypeMismatch(TypeId wantedType, TypeId givenType, TypeMismatch::Context context)`
  // (Error.cpp:1047).
  pub fn from_wanted_given_context(
    wanted_type: TypeId,
    given_type: TypeId,
    context: Context,
  ) -> Self {
    Self {
      wanted_type,
      given_type,
      reason: String::new(),
      error: None,
      context,
    }
  }
}
