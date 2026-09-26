use core::ptr::null;

use ulua_ast::records::location::Location;

use crate::{
  records::{type_function_error::TypeFunctionError, unsupported_type::UnsupportedType},
  type_aliases::{module_name_type::ModuleName, type_function_error_data::TypeFunctionErrorData},
};

impl TypeFunctionError {
  pub fn new() -> Self {
    Self {
      location: Location::default(),
      module_name: ModuleName::new(),
      data: TypeFunctionErrorData::V0(UnsupportedType { r#type: null() }),
    }
  }
}

impl Default for TypeFunctionError {
  fn default() -> Self {
    Self::new()
  }
}

impl TypeFunctionError {
  pub fn type_function_error_location_type_function_error_data(
    location: Location,
    data: TypeFunctionErrorData,
  ) -> Self {
    Self {
      location,
      module_name: ModuleName::new(),
      data,
    }
  }
}
