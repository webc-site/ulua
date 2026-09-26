use crate::{
  functions::are_equal_type_function_runtime::are_equal_are_equal_state_type_function_type_type_function_type,
  records::are_equal_state::AreEqualState,
  type_aliases::type_function_type_variant::TypeFunctionTypeVariant,
};

#[derive(Debug, Clone)]
pub struct TypeFunctionType {
  pub(crate) type_variant: TypeFunctionTypeVariant,
  pub(crate) frozen: bool,
}

impl PartialEq for TypeFunctionType {
  fn eq(&self, other: &Self) -> bool {
    let mut seen = AreEqualState::default();
    are_equal_are_equal_state_type_function_type_type_function_type(&mut seen, self, other)
  }
}
