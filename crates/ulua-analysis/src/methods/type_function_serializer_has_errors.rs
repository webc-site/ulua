use crate::{
  methods::type_function_serde_checks::builder_state_has_errors,
  records::type_function_serializer::TypeFunctionSerializer,
};

impl TypeFunctionSerializer {
  pub fn has_errors(&self) -> bool {
    builder_state_has_errors(self.state)
  }
}
