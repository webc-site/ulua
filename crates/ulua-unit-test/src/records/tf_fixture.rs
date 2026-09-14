//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.UnitTest:tests/TypeFunction.test.cpp:1745:tf_fixture`
//! Source: `tests/TypeFunction.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.test.cpp
//!   - type_ref <- method TFFixture::getBuiltins (tests/TypeFunction.test.cpp)
//!   - type_ref <- method TFFixture::getBuiltinTypeFunctions (tests/TypeFunction.test.cpp)
//! - outgoing:
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record BuiltinTypes (Analysis/include/Luau/Type.h)
//!   - calls -> method TFFixture::getBuiltins (tests/TypeFunction.test.cpp)
//!   - type_ref -> record InternalErrorReporter (Analysis/include/Luau/Error.h)
//!   - type_ref -> record UnifierSharedState (Analysis/include/Luau/UnifierSharedState.h)
//!   - type_ref -> record Normalizer (Analysis/include/Luau/Normalize.h)
//!   - type_ref -> record TypeCheckLimits (Analysis/include/Luau/TypeCheckLimits.h)
//!   - type_ref -> record TypeFunctionRuntime (Analysis/include/Luau/TypeFunctionRuntime.h)
//!   - type_ref -> record Subtyping (Analysis/include/Luau/Subtyping.h)
//!   - type_ref -> record BuiltinTypeFunctions (Analysis/include/Luau/BuiltinTypeFunctions.h)
//!   - type_ref -> record TypeFunctionContext (Analysis/include/Luau/TypeFunction.h)
//!   - translates_to -> rust_item TFFixture

use alloc::{boxed::Box, sync::Arc};
use core::{
  mem::{replace, take},
  ptr::NonNull,
};

use ulua_analysis::{
  enums::solver_mode::SolverMode,
  records::{
    builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter,
    normalizer::Normalizer, scope::Scope, subtyping::Subtyping, type_arena::TypeArena,
    type_check_limits::TypeCheckLimits, type_function_context::TypeFunctionContext,
    type_function_runtime::TypeFunctionRuntime, unifier_shared_state::UnifierSharedState,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};
use ulua_common::FFlag;
#[derive(Debug)]
pub struct TfFixture {
  pub arena: Box<TypeArena>,
  pub builtin_types: Box<BuiltinTypes>,
  pub global_scope: ScopePtr,
  pub ice: Box<InternalErrorReporter>,
  pub unifier_state: Box<UnifierSharedState>,
  pub normalizer: Box<Normalizer>,
  pub limits: Box<TypeCheckLimits>,
  pub runtime: Box<TypeFunctionRuntime>,
  pub subtyping: Box<Subtyping>,
  pub tfc: Box<TypeFunctionContext>,
}

impl Default for TfFixture {
  fn default() -> Self {
    let mut arena = Box::new(TypeArena::default());
    let mut builtin_types = Box::new(BuiltinTypes::new());
    let global_scope = Arc::new(Scope::scope_type_pack_id(builtin_types.any_type_pack));

    let mut ice = Box::new(InternalErrorReporter::default());
    let mut unifier_state = Box::new(UnifierSharedState::new(
      &mut *ice as *mut InternalErrorReporter,
    ));
    let mut normalizer = Box::new(Normalizer::new(
      &mut *arena as *mut TypeArena,
      &mut *builtin_types as *mut BuiltinTypes,
      &mut *unifier_state as *mut UnifierSharedState,
      SolverMode::New,
      false,
    ));
    let mut limits = Box::new(TypeCheckLimits::default());
    let mut runtime = Box::new(TypeFunctionRuntime::new(
      &ice,
      &limits,
      global_scope.clone(),
    ));
    let mut subtyping = Box::new(Subtyping::subtyping_owned(
      &mut *builtin_types as *mut BuiltinTypes,
      &mut *arena as *mut TypeArena,
      &mut *normalizer as *mut Normalizer,
      &mut *runtime as *mut TypeFunctionRuntime,
      &mut *ice as *mut InternalErrorReporter,
    ));

    let global_scope_ptr = Arc::as_ptr(&global_scope) as *mut Scope;
    let tfc = Box::new(TypeFunctionContext::from_components(
      NonNull::from(&mut *arena),
      NonNull::from(&mut *builtin_types),
      NonNull::new(global_scope_ptr).expect("global scope pointer should not be null"),
      NonNull::from(&mut *normalizer),
      NonNull::from(&mut *runtime),
      NonNull::from(&mut *ice),
      NonNull::from(&mut *limits),
      NonNull::from(&mut *subtyping),
    ));

    Self {
      arena,
      builtin_types,
      global_scope,
      ice,
      unifier_state,
      normalizer,
      limits,
      runtime,
      subtyping,
      tfc,
    }
  }
}

impl Drop for TfFixture {
  fn drop(&mut self) {
    let previous = FFlag::DebugLuauFreezeArena.get_global();
    FFlag::DebugLuauFreezeArena.set(true);

    let runtime = replace(
      &mut self.runtime,
      Box::new(TypeFunctionRuntime::new(
        &self.ice,
        &self.limits,
        self.global_scope.clone(),
      )),
    );
    drop(runtime);

    let arena = take(&mut self.arena);
    drop(arena);

    FFlag::DebugLuauFreezeArena.set(previous);
  }
}
