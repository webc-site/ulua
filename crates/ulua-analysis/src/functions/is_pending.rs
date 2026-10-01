use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::get_type,
  records::{
    arena_handle::alias_opt_mut, blocked_type::BlockedType, constraint_solver::ConstraintSolver,
    pending_expansion_type::PendingExpansionType,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};

/// `solver` 为会话期可空 `ConstraintSolver` 句柄（cpp `ConstraintSolver*` 原样指针身份）；
/// 解引用收口至 `alias_opt_mut`，null 折叠 None 与 `as_mut` 逐格等价，业务侧 safe 调用。对应
/// C++ `bool isPending(TypeId ty, ConstraintSolver* solver)` (`cpp/Analysis/src/TypeFunction.cpp:749`)。
pub fn is_pending(ty: TypeId, solver: *mut ConstraintSolver) -> bool {
  if let Some(tfit) = get_type::get::<TypeFunctionInstanceType>(ty)
    && tfit.state == TypeFunctionInstanceState::Unsolved
  {
    return true;
  }

  if get_type::get::<BlockedType>(ty).is_some() {
    return true;
  }

  if get_type::get::<PendingExpansionType>(ty).is_some() {
    return true;
  }

  // C++ `solver` 可空：alias_opt_mut 把 null 折叠为 None，命中时按模块契约
  // 重建独占可变借用，仅供 `has_unresolved_constraints` 使用。
  if let Some(solver) = alias_opt_mut(solver) {
    return solver.has_unresolved_constraints(ty);
  }

  false
}
