use crate::{
  records::type_function_type::TypeFunctionType,
  type_aliases::type_function_type_variant::TypeFunctionTypeVariant,
};

impl TypeFunctionType {
  pub fn new(type_variant: TypeFunctionTypeVariant) -> Self {
    Self {
      type_variant,
      frozen: false,
    }
  }
}
