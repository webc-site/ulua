use crate::type_aliases::type_function_singleton_variant::TypeFunctionSingletonVariant;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeFunctionSingletonType {
  pub variant: TypeFunctionSingletonVariant,
}
