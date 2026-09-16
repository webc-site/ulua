use crate::{
  functions::are_equal_type_function_runtime_alt_m::are_equal_are_equal_state_type_function_type_type_function_type,
  records::{are_equal_state::AreEqualState, type_function_type::TypeFunctionType},
};

impl TypeFunctionType {
  pub fn operator_eq(&self, rhs: &TypeFunctionType) -> bool {
    let mut seen = AreEqualState::default();
    are_equal_are_equal_state_type_function_type_type_function_type(&mut seen, self, rhs)
  }
}
