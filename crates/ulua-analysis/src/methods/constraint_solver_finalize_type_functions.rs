//! `void ConstraintSolver::finalizeTypeFunctions()`
//! (`Analysis/src/ConstraintSolver.cpp:767-784`, hand-ported faithfully).

use alloc::vec::Vec;
use core::ptr::NonNull;

use crate::{
  functions::{follow_type, get_type, reduce_type_functions_type_function::reduce_type_functions},
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver,
    type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};
impl ConstraintSolver {
  pub fn constraint_solver_finalize_type_functions(&mut self) {
    // At this point, we've generalized. Let's try to finish reducing as much as we can, we'll leave warning to the typechecker
    let entries: Vec<(TypeId, *const Constraint)> = self
      .type_functions_to_finalize
      .iter()
      .map(|(t, constraint)| (*t, *constraint))
      .collect();

    for (t, constraint) in entries {
      let ty = follow_type::follow(t);
      if get_type::get::<TypeFunctionInstanceType>(ty).is_some() {
        // Safety: constraint 是 type_functions_to_finalize 里保存的 *const
        // Constraint 句柄，其对象由 push_constraint 经 ConstraintArena bump 分配
        // （块地址不移动），entries 快照只拷贝指针值；finalize 阶段 arena 与求解器
        // 同活，读取 scope/location 两个字段为纯只读，无别名冲突。
        let scope = unsafe { (*constraint).scope };
        // Safety: 同上——location 是同一存活 Constraint 节点的只读字段，借用止于
        // 本行拷贝。
        let location = unsafe { (*constraint).location };

        let mut context = TypeFunctionContext::from_solver(
          NonNull::new(self as *mut ConstraintSolver).expect("&mut self 转裸指针恒非空"),
          NonNull::new(scope)
            .expect("scope 即 constraint.scope（cpp NotNull<Scope> 契约），恒非空"),
          NonNull::new(constraint as *mut Constraint)
            .expect("正在处理的约束借自活引用，转裸指针恒非空"),
          NonNull::new(self.subtyping)
            .expect("subtyping 由构造期 Handle(NonNull) 接线为 *mut，恒非空"),
        );

        // cpp `reduceTypeFunctions(t, location, &context, true)`：本帧的栈局 context
        // 显然活过本次调用，直接借出即可（原 `NonNull::new(&mut context …).expect(…)`
        // 的判空/裸指针往返属纯冗余）。
        let result = reduce_type_functions(t, location, &mut context, true);

        for r in result.reduced_types.iter() {
          self.unblock_type_id_location(*r, location);
        }
        for r in result.reduced_packs.iter() {
          self.unblock_type_pack_id_location(*r, location);
        }
      }
    }
  }
}
