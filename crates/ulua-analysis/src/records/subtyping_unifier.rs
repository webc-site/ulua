use crate::records::{
  arena_handle::Handle, builtin_types::BuiltinTypes,
  internal_error_reporter::InternalErrorReporter, type_arena::TypeArena,
};

#[derive(Debug, Clone)]
pub struct SubtypingUnifier {
  pub(crate) arena: Handle<TypeArena>,
  pub(crate) builtin_types: Handle<BuiltinTypes>,
  pub reporter: Handle<InternalErrorReporter>,
}

// `subtyping_unifier` (ctor) / `dispatch_constraints` / `dispatch_one_constraint`
// live in their own method node files (methods/subtyping_unifier_*.rs).
