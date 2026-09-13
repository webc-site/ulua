use alloc::vec::Vec;

use crate::{
  records::{
    builtin_types::BuiltinTypes, scope::Scope, substitution::Substitution, type_level::TypeLevel,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct ReplaceGenerics {
  pub base: Substitution,
  pub builtin_types: *mut BuiltinTypes,
  pub level: TypeLevel,
  pub scope: *mut Scope,
  pub generics: Vec<TypeId>,
  pub generic_packs: Vec<TypePackId>,
}
