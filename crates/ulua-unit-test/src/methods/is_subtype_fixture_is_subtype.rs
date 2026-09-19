//! Source: `tests/Fixture.cpp`

use alloc::sync::Arc;

use ulua_analysis::{
  enums::solver_mode::SolverMode,
  functions::is_subtype_normalize_alt_b::is_subtype,
  records::{
    normalizer::Normalizer, type_arena::TypeArena, type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime, unifier_shared_state::UnifierSharedState,
  },
  type_aliases::type_id::TypeId,
};
use ulua_common::fflag;

use crate::records::is_subtype_fixture::IsSubtypeFixture;
impl IsSubtypeFixture {
  pub fn is_subtype(&mut self, a: TypeId, b: TypeId) -> bool {
    let module = self.base.get_main_module(false);
    assert!(!module.is_null(), "isSubtype: expected main module");

    let module = unsafe { &*module };
    assert!(
      module.has_module_scope(),
      "isSubtype: module scope data is not available"
    );

    let scope = module.get_module_scope();
    let mut shared_state = UnifierSharedState::new(&mut self.base.ice as *mut _);
    let solver_mode = if fflag::DebugLuauForceOldSolver.get() {
      SolverMode::Old
    } else {
      SolverMode::New
    };
    let mut normalizer = Normalizer::new(
      &mut self.base.arena as *mut _,
      self.base.builtin_types,
      &mut shared_state as *mut _,
      solver_mode,
      false,
    );

    let mut arena = TypeArena::default();
    let limits = TypeCheckLimits::default();
    let mut type_function_runtime =
      TypeFunctionRuntime::new(&self.base.ice, &limits, scope.clone());

    is_subtype(
      a,
      b,
      &mut arena as *mut _,
      self.base.builtin_types,
      Arc::as_ptr(&scope) as *mut _,
      &mut normalizer as *mut _,
      &mut type_function_runtime as *mut _,
      &mut self.base.ice as *mut _,
    )
  }
}
