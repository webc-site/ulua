use crate::{
  records::{
    extern_type::ExternType, instantiation_queuer_deprecated::InstantiationQueuerDeprecated,
    pending_expansion_type::PendingExpansionType, reduce_constraint::ReduceConstraint,
    type_alias_expansion_constraint::TypeAliasExpansionConstraint,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::{constraint_v::ConstraintV, type_id::TypeId},
};

impl InstantiationQueuerDeprecated {
  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    _ty: TypeId,
    _petv: &PendingExpansionType,
  ) -> bool {
    // Safety: self.solver 由 ConstraintSolver::try_dispatch 以 `self as *mut` 接线
    // （queuer 是该栈帧局部对象，寿命嵌套于 solver 的 &mut 借用内），指针非空、
    // 对齐且存活；重建的 &mut 仅服务紧随的 push_constraint 调用，借用止于该语句
    // ——单线程串行遍历中此刻无其它存活的可变/共享借用指向 solver 状态。
    let solver = unsafe { &mut *self.solver };
    solver.push_constraint(
      self.scope,
      self.location,
      ConstraintV::TypeAliasExpansion(TypeAliasExpansionConstraint { target: _ty }),
    );
    false
  }

  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    _tfit: &TypeFunctionInstanceType,
  ) -> bool {
    // Safety: 同 visit_type_id_pending_expansion_type——solver 构造期从驱动栈帧
    // 接线非空，重建可变借用只为一次 push_constraint，串行执行下无别名重叠。
    let solver = unsafe { &mut *self.solver };
    solver.push_constraint(
      self.scope,
      self.location,
      ConstraintV::Reduce(ReduceConstraint { ty }),
    );
    true
  }

  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
    false
  }
}
