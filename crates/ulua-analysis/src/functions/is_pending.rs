use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::get_type,
  records::{
    blocked_type::BlockedType, constraint_solver::ConstraintSolver,
    pending_expansion_type::PendingExpansionType,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};

/// # Safety
/// `solver` 须指向调用方持有的存活 `ConstraintSolver`（非空、对齐，随其所有者长寿）；本函数
/// 只读经 `self`/`solver` 查询 `ty` 的阻塞状态，不写。调用方单线程独占该 solver，无并发写。对应
/// C++ `bool isPending(TypeId ty, ConstraintSolver* solver)` (`cpp/Analysis/src/TypeFunction.cpp:749`)。
pub unsafe fn is_pending(ty: TypeId, solver: *mut ConstraintSolver) -> bool {
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

  // Safety: `solver` 为可空裸指针，`solver.as_mut()` 把 null 折叠为 None（不产生引用、不解引用）；
  // 命中 Some 时它指向会话期存活的 ConstraintSolver（由 TypeFunctionContext 持有、比本次调用长寿）。
  // 本函数处于单线程串行的归约调用栈中、此刻无其它对 solver 的存活借用，故重建的 `&mut` 独占成立，
  // 仅供 `has_unresolved_constraints` 使用。
  if let Some(solver) = unsafe { solver.as_mut() } {
    // SAFETY: C++ `solver` 为可空指针；as_mut 自带判空，指向会话期
    // ConstraintSolver，生命周期由 TypeFunctionContext 持有。
    return solver.has_unresolved_constraints(ty);
  }

  false
}
