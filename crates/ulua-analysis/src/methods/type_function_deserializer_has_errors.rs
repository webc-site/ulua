use ulua_common::FFlag;

use crate::records::type_function_deserializer::TypeFunctionDeserializer;
impl TypeFunctionDeserializer {
  pub fn has_errors(&self) -> bool {
    if self.state.is_null() {
      return false;
    }

    unsafe {
      if FFlag::LuauTypeFunctionStructuredErrors.get() {
        !(*self.state).errors.is_empty()
      } else {
        !(*self.state).errors_deprecated.is_empty()
      }
    }
  }
}
