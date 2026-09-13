use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant;

#[derive(Debug, Clone)]
pub struct TypeFunctionType {
  pub(crate) type_variant: TypeFunctionTypeVariant,
  pub(crate) frozen: bool,
}
