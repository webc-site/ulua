//! `instantiation_queuer` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{string::String, vec::Vec};
use core::ptr::NonNull;

use ulua_ast::records::location::Location;

use crate::{
  records::{
    constraint_solver::ConstraintSolver,
    extern_type::ExternType,
    instantiation_queuer::InstantiationQueuer,
    instantiation_queuer_deprecated::InstantiationQueuerDeprecated,
    iterative_type_visitor::{IterativeTypeVisitor, IterativeTypeVisitorTrait},
    pending_expansion_type::PendingExpansionType,
    reduce_constraint::ReduceConstraint,
    scope::Scope,
    type_alias_expansion_constraint::TypeAliasExpansionConstraint,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::{
    constraint_v::ConstraintV, seen_set_iterative_type_visitor::SeenSet, type_id::TypeId,
  },
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

impl InstantiationQueuer {
  pub fn new(scope: NonNull<Scope>, location: &Location, solver: *mut ConstraintSolver) -> Self {
    let mut visitor = InstantiationQueuer {
      base: IterativeTypeVisitor {
        seen: SeenSet::default(),
        work_queue: Vec::new(),
        parent_cursor: -1,
        work_cursor: 0,
        visitor_name: String::from("InstantiationQueuer"),
        skip_bound_types: true,
        visit_once: true,
      },
      solver,
      scope,
      location: *location,
    };
    visitor
      .base
      .iterative_type_visitor_string_bool_bool("InstantiationQueuer", true, true);
    visitor
  }
}

impl IterativeTypeVisitorTrait for InstantiationQueuer {
  fn visitor_base(&mut self) -> &mut IterativeTypeVisitor {
    &mut self.base
  }

  fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    petv: &PendingExpansionType,
  ) -> bool {
    InstantiationQueuer::visit_type_id_pending_expansion_type(self, ty, petv)
  }

  fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    InstantiationQueuer::visit_type_id_type_function_instance_type(self, ty, tfit)
  }

  fn visit_type_id_extern_type(&mut self, ty: TypeId, etv: &ExternType) -> bool {
    InstantiationQueuer::visit_type_id_extern_type(self, ty, etv)
  }
}

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
