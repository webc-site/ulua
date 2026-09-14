use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::get_type_alt_j::get_type_id,
  records::{
    blocked_type::BlockedType, constraint_solver::ConstraintSolver,
    pending_expansion_type::PendingExpansionType,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};

/// # Safety
/// 调用方须保证 `solver` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn is_pending(ty: TypeId, solver: *mut ConstraintSolver) -> bool {
  if let Some(tfit) = get_type_id::<TypeFunctionInstanceType>(ty)
    && tfit.state == TypeFunctionInstanceState::Unsolved
  {
    return true;
  }

  if get_type_id::<BlockedType>(ty).is_some() {
    return true;
  }

  if get_type_id::<PendingExpansionType>(ty).is_some() {
    return true;
  }

  if let Some(solver) = unsafe { solver.as_mut() } {
    // SAFETY: C++ `solver` 为可空指针；as_mut 自带判空，指向会话期
    // ConstraintSolver，生命周期由 TypeFunctionContext 持有。
    return solver.has_unresolved_constraints(ty);
  }

  false
}
