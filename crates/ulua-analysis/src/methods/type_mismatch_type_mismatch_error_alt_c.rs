//! Source: `Analysis/src/Error.cpp`

use alloc::string::String;

use crate::{
  enums::context_error::Context, records::type_mismatch::TypeMismatch,
  type_aliases::type_id::TypeId,
};
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
}
