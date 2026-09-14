use crate::records::{
  builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter,
  type_arena::TypeArena,
};

#[derive(Debug, Clone)]
pub struct SubtypingUnifier {
  pub(crate) arena: *mut TypeArena,
  pub(crate) builtin_types: *mut BuiltinTypes,
  pub reporter: *mut InternalErrorReporter,
}

// `subtyping_unifier` (ctor) / `dispatch_constraints` / `dispatch_one_constraint`
// live in their own method node files (methods/subtyping_unifier_*.rs).
