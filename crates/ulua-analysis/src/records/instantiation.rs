use crate::records::{
  arena_handle::Handle, builtin_types::BuiltinTypes, replace_generics::ReplaceGenerics,
  scope::Scope, substitution::Substitution, type_level::TypeLevel,
};

#[derive(Debug, Clone)]
pub struct Instantiation {
  pub base: Substitution,
  pub builtin_types: Handle<BuiltinTypes>,
  pub level: TypeLevel,
  pub scope: *mut Scope,
  pub reusable_replace_generics: ReplaceGenerics,
}
