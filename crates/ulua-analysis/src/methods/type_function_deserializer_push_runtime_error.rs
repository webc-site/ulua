use alloc::string::String;

use ulua_ast::records::location::Location;
use ulua_common::fflag;

use crate::{
  records::{
    runtime_error::RuntimeError, type_function_deserializer::TypeFunctionDeserializer,
    type_function_error::TypeFunctionError,
  },
  type_aliases::{module_name_type::ModuleName, type_function_error_data::TypeFunctionErrorData},
};
impl TypeFunctionDeserializer {
  pub fn push_runtime_error(&mut self, message: String) {
    if self.state.is_null() {
      return;
    }

    unsafe {
      if fflag::LuauTypeFunctionStructuredErrors.get() {
        (*self.state).errors.push(TypeFunctionError {
          location: Location::default(),
          module_name: ModuleName::new(),
          data: TypeFunctionErrorData::V2(RuntimeError::new(message)),
        });
      } else {
        (*self.state).errors_deprecated.push(message);
      }
    }
  }
}
