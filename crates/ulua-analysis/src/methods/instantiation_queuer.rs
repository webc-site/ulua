//! `instantiation_queuer` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{string::String, vec::Vec};
use core::ptr::NonNull;

use ulua_ast::records::location::Location;

use crate::{
  records::{
    arena_handle::alias,
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
    let solver = alias(self.solver);
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
    let solver = alias(self.solver);
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
    let solver = alias(self.solver);
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
    let solver = alias(self.solver);
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
