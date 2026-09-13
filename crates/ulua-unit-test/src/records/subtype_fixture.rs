use alloc::{boxed::Box, sync::Arc};
// Embeds TypeFunctionRuntime (non-copyable in C++) — no Clone.
use core::mem::replace;
use core::{mem::take, ptr::NonNull};

use ulua_analysis::{
  enums::solver_mode::SolverMode,
  records::{
    builtin_type_functions::BuiltinTypeFunctions, builtin_types::BuiltinTypes,
    global_types::GlobalTypes, internal_error_reporter::InternalErrorReporter,
    normalizer::Normalizer, scope::Scope, subtyping::Subtyping, type_arena::TypeArena,
    type_check_limits::TypeCheckLimits, type_function_runtime::TypeFunctionRuntime,
    unifier_shared_state::UnifierSharedState,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};
use ulua_common::FFlag;

use crate::records::fixture::Fixture;
#[derive(Debug)]
pub struct SubtypeFixture {
  pub builtin_type_functions: BuiltinTypeFunctions,
  pub subtyping: Box<Subtyping>,
  pub module_scope: ScopePtr,
  pub root_scope: ScopePtr,
  pub type_function_runtime: Box<TypeFunctionRuntime>,
  pub limits: TypeCheckLimits,
  pub normalizer: Box<Normalizer>,
  pub shared_state: Box<UnifierSharedState>,
  pub ice_reporter: Box<InternalErrorReporter>,
  pub arena: Box<TypeArena>,
  pub global_types: Box<GlobalTypes>,
  pub builtin_types: Box<BuiltinTypes>,
  pub base: Box<Fixture>,
}

impl Default for SubtypeFixture {
  fn default() -> Self {
    let mut base = Box::new(Fixture::default());
    let mut builtin_types = Box::new(BuiltinTypes::new());
    base.builtin_types = &mut *builtin_types;
    let global_types = Box::new(GlobalTypes::new(
      NonNull::from(&mut *builtin_types),
      SolverMode::New,
    ));

    let mut arena = Box::new(TypeArena::default());
    let mut ice_reporter = Box::new(InternalErrorReporter::default());
    let mut shared_state = Box::new(UnifierSharedState::new(
      &mut *ice_reporter as *mut InternalErrorReporter,
    ));

    let mut normalizer = Box::new(Normalizer::new(
      &mut *arena as *mut TypeArena,
      &mut *builtin_types as *mut BuiltinTypes,
      &mut *shared_state as *mut UnifierSharedState,
      SolverMode::New,
      false,
    ));

    let limits = TypeCheckLimits::default();
    let root_scope = Arc::new(Scope::scope_type_pack_id(builtin_types.empty_type_pack));
    let module_scope = Arc::new(Scope::new(&root_scope, 0));
    let mut type_function_runtime = Box::new(TypeFunctionRuntime::new(
      &ice_reporter,
      &limits,
      root_scope.clone(),
    ));

    let subtyping = Box::new(Subtyping::subtyping_owned(
      &mut *builtin_types as *mut BuiltinTypes,
      &mut *arena as *mut TypeArena,
      &mut *normalizer as *mut Normalizer,
      &mut *type_function_runtime as *mut TypeFunctionRuntime,
      &mut *ice_reporter as *mut InternalErrorReporter,
    ));

    Self {
      base,
      global_types,
      builtin_types,
      arena,
      ice_reporter,
      shared_state,
      normalizer,
      limits,
      type_function_runtime,
      root_scope,
      module_scope,
      subtyping,
      builtin_type_functions: BuiltinTypeFunctions::new(),
    }
  }
}

impl Drop for SubtypeFixture {
  fn drop(&mut self) {
    let previous = FFlag::DebugLuauFreezeArena.get_global();
    FFlag::DebugLuauFreezeArena.set(true);

    let runtime = replace(
      &mut self.type_function_runtime,
      Box::new(TypeFunctionRuntime::new(
        &self.ice_reporter,
        &self.limits,
        self.root_scope.clone(),
      )),
    );
    drop(runtime);

    let arena = take(&mut self.arena);
    drop(arena);

    FFlag::DebugLuauFreezeArena.set(previous);
  }
}
