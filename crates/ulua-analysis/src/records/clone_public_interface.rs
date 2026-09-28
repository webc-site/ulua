use crate::{
  enums::solver_mode::SolverMode,
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, module::Module, substitution::Substitution,
  },
};

#[derive(Debug, Clone)]
pub struct ClonePublicInterface {
  pub(crate) base: Substitution,
  pub(crate) builtin_types: Handle<BuiltinTypes>,
  pub(crate) module: *mut Module,
  pub(crate) solver_mode: SolverMode,
  pub(crate) internal_type_escaped: bool,
}
