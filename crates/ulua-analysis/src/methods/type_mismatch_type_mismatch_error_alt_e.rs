//! Source: `Analysis/src/Error.cpp`

use alloc::string::String;

use crate::{
  enums::context_error::Context, records::type_mismatch::TypeMismatch,
  type_aliases::type_id::TypeId,
};
impl TypeMismatch {
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
