use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  records::type_function_instance_type::TypeFunctionInstanceType,
};

impl TypeFunctionInstanceType {
  pub fn state(&self) -> TypeFunctionInstanceState {
    self.state
  }
}
