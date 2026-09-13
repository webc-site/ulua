use alloc::string::String;
use core::ptr::null;

use ulua_ast::records::location::Location;

use crate::{
  records::{type_function_error::TypeFunctionError, unsupported_type::UnsupportedType},
  type_aliases::type_function_error_data::TypeFunctionErrorData,
};
impl TypeFunctionError {
  pub fn new() -> Self {
    Self {
      location: Location::default(),
      module_name: String::new(),
      data: TypeFunctionErrorData::V0(UnsupportedType { r#type: null() }),
    }
  }
}

impl Default for TypeFunctionError {
  fn default() -> Self {
    Self::new()
  }
}
