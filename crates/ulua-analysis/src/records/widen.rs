use crate::records::{
  arena_handle::Handle, builtin_types::BuiltinTypes, substitution::Substitution,
};

#[derive(Debug, Clone)]
pub struct Widen {
  pub(crate) base: Substitution,
  pub(crate) builtin_types: Handle<BuiltinTypes>,
}
