use crate::{
  records::{
    builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter, scope::Scope,
    substitution::Substitution,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct Anyification {
  pub base: Substitution,
  pub scope: *mut Scope,
  pub builtin_types: *const BuiltinTypes,
  pub ice_handler: *mut InternalErrorReporter,
  pub any_type: TypeId,
  pub any_type_pack: TypePackId,
  pub normalization_too_complex: bool,
}
