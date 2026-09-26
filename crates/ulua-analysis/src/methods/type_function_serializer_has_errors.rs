use ulua_common::fflag;

use crate::records::type_function_serializer::TypeFunctionSerializer;
impl TypeFunctionSerializer {
  pub fn has_errors(&self) -> bool {
    if self.state.is_null() {
      return false;
    }

    unsafe {
      if fflag::LuauTypeFunctionStructuredErrors.get() {
        !(*self.state).errors.is_empty()
      } else {
        !(*self.state).errors_deprecated.is_empty()
      }
    }
  }
}
