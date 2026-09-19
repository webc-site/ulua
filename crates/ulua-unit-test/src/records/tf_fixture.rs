//! Source: `tests/TypeFunction.test.cpp`

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
use ulua_common::fflag;

use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

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
    // thread-local override 而非全局 set/restore：并行 libtest 下全局写会
    // 干扰其他测试线程读该 flag，且多个 fixture 的 save/restore 交叉会把
    // 全局值永久卡在 true。override 仅本线程可见，析构代码经 get() 命中。
    let _freeze = ScopedFastFlag::new(&fflag::DebugLuauFreezeArena, true);

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
  }
}
