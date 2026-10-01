use crate::{
  records::{arena_handle::Handle, builtin_types::BuiltinTypes, substitution::Substitution},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct Anyification {
  pub base: Substitution,
  pub builtin_types: Handle<BuiltinTypes>,
  pub any_type: TypeId,
  pub any_type_pack: TypePackId,
  pub normalization_too_complex: bool,
}
