use crate::{
  records::type_function_type_pack_var::TypeFunctionTypePackVar,
  type_aliases::type_function_type_pack_variant::TypeFunctionTypePackVariant,
};

impl TypeFunctionTypePackVar {
  pub fn new(type_variant: TypeFunctionTypePackVariant) -> Self {
    Self { type_variant }
  }
}
