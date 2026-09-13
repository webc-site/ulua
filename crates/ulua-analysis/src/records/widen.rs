use crate::records::{builtin_types::BuiltinTypes, substitution::Substitution};

#[derive(Debug, Clone)]
pub struct Widen {
  pub(crate) base: Substitution,
  pub(crate) builtin_types: *const BuiltinTypes,
}
