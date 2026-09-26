use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_ast::records::ast_name::AstName;

use crate::{
  records::{
    arena_id::ArenaId, builtin_types::BuiltinTypes, constraint::Constraint,
    constraint_solver::ConstraintSolver, internal_error_reporter::InternalErrorReporter,
    normalizer::Normalizer, scope::Scope, subtyping::Subtyping, type_arena::TypeArena,
    type_check_limits::TypeCheckLimits, type_function_runtime::TypeFunctionRuntime,
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
    // SAFETY: `builtins` is valid and non-null for the lifetime of TypeFunctionContext.
    unsafe { self.builtins.as_ref() }
  }

  #[inline]
  pub fn ice(&self) -> &InternalErrorReporter {
    // SAFETY: `ice` is valid and non-null for the lifetime of TypeFunctionContext.
    unsafe { self.ice.as_ref() }
  }

  #[inline]
  pub fn arena_mut(&mut self) -> &mut TypeArena {
    // SAFETY: `arena` is valid and uniquely accessed for the lifetime of TypeFunctionContext.
    unsafe { self.arena.as_mut() }
  }

  #[inline]
  pub fn normalizer_mut(&mut self) -> &mut Normalizer {
    // SAFETY: `normalizer` is valid and uniquely accessed for the lifetime of TypeFunctionContext.
    unsafe { self.normalizer.as_mut() }
  }

  #[inline]
  pub fn solver_mut(&mut self) -> Option<&mut ConstraintSolver> {
    // SAFETY: `solver` is either null or points to a valid ConstraintSolver.
    unsafe { self.solver.as_mut() }
  }

  /// 归属比较用的 arena 身份（cpp `ctx->arena` 指针相等判据的值化形态）。
  /// 解引用收口在本方法一处，契约与 [`Self::arena`] 各装配/取用点相同。
  pub(crate) fn arena_id(&self) -> ArenaId {
    // SAFETY: `arena` 为装配期接线的存活、地址稳定 `TypeArena`（单线程驱动，
    // 借用期内无并存可变别名），此处仅按值读出 `arena_id`。
    unsafe { (*self.arena.as_ptr()).arena_id }
  }
}

// NOTE: the two ctors and `push_constraint` are implemented in their own
// method files (so this record stays a pure data definition):
//   - methods/type_function_context_type_function_context_type_function.rs
//     (`from_components` — the arena/builtins/... ctor, TypeFunction.h:60)
//   - methods/type_function_context_type_function_context_builtin_type_functions.rs
//     (`from_solver` — the ConstraintSolver ctor, BuiltinTypeFunctions.cpp:362)
//   - methods/type_function_context_push_constraint.rs (`push_constraint`)
