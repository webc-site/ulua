use crate::{
  records::{
    extern_type::ExternType, instantiation_queuer::InstantiationQueuer,
    pending_expansion_type::PendingExpansionType, reduce_constraint::ReduceConstraint,
    type_alias_expansion_constraint::TypeAliasExpansionConstraint,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::{constraint_v::ConstraintV, type_id::TypeId},
};

impl InstantiationQueuer {
  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    _petv: &PendingExpansionType,
  ) -> bool {
    // Safety: `self.solver` 由 `InstantiationQueuer::new` 保存，其源头是持有者同一调用栈内
    // 的 `self as *mut ConstraintSolver`（见 constraint_solver_try_dispatch 中的构造点），
    // 故必非空、对齐，并且至少活到 `queuer.run_type_id(..)` 返回。遍历期间外层只在 queuer
    // 上推进（不再触碰 solver 的字段），单线程串行下这一重建的 `&mut` 是唯一可变借用，
    // 因此 `push_constraint` 对 solver 状态（约束队列、依赖图）的写入合法。
    let solver = unsafe { &mut *self.solver };
    solver.push_constraint(
      self.scope,
      self.location,
      ConstraintV::TypeAliasExpansion(TypeAliasExpansionConstraint { target: ty }),
    );
    false
  }

  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    _tfit: &TypeFunctionInstanceType,
  ) -> bool {
    // Safety: 同 `visit_type_id_pending_expansion_type` —— solver 指针源自外层
    // `&mut ConstraintSolver` 的 `self as *mut _`，非空且在 queuer 运行期存活；本方法是
    // 迭代式 visitor 的串行回调，重建的可变借用是该时刻唯一指向 solver 的活动借用，
    // 因而 `push_constraint` / `type_functions_to_finalize` 的读写（含随后
    // `constraint.as_ptr()` 记录的 NonNull）不会与并存的 `&`/`&mut` 冲突。
    let solver = unsafe { &mut *self.solver };
    // 对齐 C++：同一 TypeFunctionInstanceType 只登记一次，否则 finalize
    // 阶段会为同一类型重复入队 Reduce 约束。
    if solver.type_functions_to_finalize.find(&ty).is_none() {
      let constraint = solver.push_constraint(
        self.scope,
        self.location,
        ConstraintV::Reduce(ReduceConstraint { ty }),
      );
      solver
        .type_functions_to_finalize
        .insert(ty, constraint.as_ptr());
    }
    true
  }

  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
    false
  }
}
