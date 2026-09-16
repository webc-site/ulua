/// MagicFunction is an abstract base class in C++. In Rust, we model this as a struct
/// containing function pointers (vtable) to allow custom typechecking logic for builtins.
use core::fmt::Debug;
use core::fmt::{Formatter, Result};

use crate::{
  records::{
    magic_function_call_context::MagicFunctionCallContext,
    magic_function_type_check_context::MagicFunctionTypeCheckContext,
    magic_refinement_context::MagicRefinementContext,
  },
  type_aliases::old_solver_handler::OldSolverHandler,
};
#[derive(Clone)]
pub struct MagicFunction {
  pub(crate) handle_old_solver: OldSolverHandler,

  pub(crate) infer: fn(&MagicFunctionCallContext) -> bool,

  pub(crate) refine: fn(&MagicRefinementContext),

  pub(crate) type_check: fn(&MagicFunctionTypeCheckContext) -> bool,
}

impl MagicFunction {
  /// Build a `MagicFunction` vtable from its four handler function pointers.
  /// This is the public analog of constructing a `MagicFunction` subclass
  /// (e.g. the test-only `MagicInstanceIsA`) outside this crate, where the
  /// fields are not directly accessible.
  pub fn from_handlers(
    handle_old_solver: OldSolverHandler,
    infer: fn(&MagicFunctionCallContext) -> bool,
    refine: fn(&MagicRefinementContext),
    type_check: fn(&MagicFunctionTypeCheckContext) -> bool,
  ) -> Self {
    MagicFunction {
      handle_old_solver,
      infer,
      refine,
      type_check,
    }
  }
}

impl Debug for MagicFunction {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("MagicFunction").finish_non_exhaustive()
  }
}

unsafe impl Send for MagicFunction {}
unsafe impl Sync for MagicFunction {}
