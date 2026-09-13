use alloc::string::String;

use ulua_ast::records::location::Location;

use crate::{
  records::type_function_error::TypeFunctionError,
  type_aliases::type_function_error_data::TypeFunctionErrorData,
};
impl TypeFunctionError {
  pub fn type_function_error_location_type_function_error_data(
    location: Location,
    data: TypeFunctionErrorData,
  ) -> Self {
    Self {
      location,
      module_name: String::new(),
      data,
    }
  }
}
