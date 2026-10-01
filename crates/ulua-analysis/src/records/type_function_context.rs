use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_ast::records::ast_name::AstName;

use crate::{
  records::{
    arena_handle::{alias, alias_ref},
    arena_id::ArenaId,
    builtin_types::BuiltinTypes,
    constraint::Constraint,
    constraint_solver::ConstraintSolver,
    internal_error_reporter::InternalErrorReporter,
    normalizer::Normalizer,
    scope::Scope,
    subtyping::Subtyping,
    type_arena::TypeArena,
    type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime,
  },
  type_aliases::type_id::TypeId,
};
#[derive(Debug, Clone)]
pub struct TypeFunctionContext {
  pub arena: NonNull<TypeArena>,
  pub builtins: NonNull<BuiltinTypes>,
  pub scope: NonNull<Scope>,
  pub normalizer: NonNull<Normalizer>,
  pub type_function_runtime: NonNull<TypeFunctionRuntime>,
  pub ice: NonNull<InternalErrorReporter>,
  pub limits: NonNull<TypeCheckLimits>,
  pub subtyping: NonNull<Subtyping>,
  pub solver: *mut ConstraintSolver,
  pub constraint: *const Constraint,
  pub user_func_name: Option<AstName>,
  pub fresh_instances: Vec<TypeId>,
}

impl TypeFunctionContext {
  #[inline]
  pub fn builtins(&self) -> &BuiltinTypes {
    alias_ref(self.builtins.as_ptr())
  }

  #[inline]
  pub fn ice(&self) -> &InternalErrorReporter {
    alias_ref(self.ice.as_ptr())
  }

  #[inline]
  pub fn arena_mut(&mut self) -> &mut TypeArena {
    alias(self.arena.as_ptr())
  }

  #[inline]
  pub fn normalizer_mut(&mut self) -> &mut Normalizer {
    alias(self.normalizer.as_ptr())
  }

  /// 归属比较用的 arena 身份（cpp `ctx->arena` 指针相等判据的值化形态）。
  /// 解引用收口在本方法一处，契约与 [`Self::arena`] 各装配/取用点相同。
  pub(crate) fn arena_id(&self) -> ArenaId {
    alias_ref(self.arena.as_ptr()).arena_id
  }
}

// NOTE: the two ctors and `push_constraint` are implemented in their own
// method files (so this record stays a pure data definition):
//   - methods/type_function_context_type_function_context_type_function.rs
//     (`from_components` — the arena/builtins/... ctor, TypeFunction.h:60)
//   - methods/type_function_context_type_function_context_builtin_type_functions.rs
//     (`from_solver` — the ConstraintSolver ctor, BuiltinTypeFunctions.cpp:362)
//   - methods/type_function_context_push_constraint.rs (`push_constraint`)
