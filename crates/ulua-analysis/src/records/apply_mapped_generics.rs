/// C++ `ApplyMappedGenerics` (`Subtyping.cpp`): a `Substitution` subclass, so it
/// embeds `base: Substitution` and inherits `substitute` (whose traversal
/// virtual-dispatches into the `isDirty` / `clean` / `ignoreChildren` overrides
/// installed via [`SubstitutionVtable`](crate::records::tarjan::SubstitutionVtable)).
use crate::records::builtin_types::BuiltinTypes;
use crate::records::{
  internal_error_reporter::InternalErrorReporter, substitution::Substitution,
  subtyping_environment::SubtypingEnvironment, type_arena::TypeArena,
};
#[derive(Debug, Clone)]
pub struct ApplyMappedGenerics {
  pub(crate) base: Substitution,
  pub(crate) builtin_types: *mut BuiltinTypes,
  pub(crate) arena: *mut TypeArena,
  pub(crate) ice_reporter: *mut InternalErrorReporter,
  pub(crate) env: *mut SubtypingEnvironment,
}
