use crate::{
  functions::are_equal_type_function_runtime::are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var,
  records::are_equal_state::AreEqualState,
  type_aliases::type_function_type_pack_variant::TypeFunctionTypePackVariant,
};

#[derive(Debug, Clone)]
pub struct TypeFunctionTypePackVar {
  pub(crate) type_variant: TypeFunctionTypePackVariant,
}

impl PartialEq for TypeFunctionTypePackVar {
  fn eq(&self, other: &Self) -> bool {
    let mut seen = AreEqualState {
      seen: Default::default(),
      recursion_count: 0,
    };
    are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var(
      &mut seen, self, other,
    )
  }
}
