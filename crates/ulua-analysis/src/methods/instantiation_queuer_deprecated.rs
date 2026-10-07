//! `instantiation_queuer_deprecated` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::string::String;
use core::ptr::NonNull;

use ulua_ast::records::location::Location;
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    constraint_solver::ConstraintSolver,
    extern_type::ExternType,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    instantiation_queuer_deprecated::InstantiationQueuerDeprecated,
    pending_expansion_type::PendingExpansionType,
    scope::Scope,
    type_function_instance_type::TypeFunctionInstanceType,
    type_once_visitor::TypeOnceVisitor,
    visit_key::VisitKey,
  },
  type_aliases::type_id::TypeId,
};

impl GenericTypeVisitorTrait for InstantiationQueuerDeprecated {
  type Seen = DenseHashSet<VisitKey>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    petv: &PendingExpansionType,
  ) -> bool {
    InstantiationQueuerDeprecated::visit_type_id_pending_expansion_type(self, ty, petv)
  }

  fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    InstantiationQueuerDeprecated::visit_type_id_type_function_instance_type(self, ty, tfit)
  }

  fn visit_type_id_extern_type(&mut self, ty: TypeId, etv: &ExternType) -> bool {
    InstantiationQueuerDeprecated::visit_type_id_extern_type(self, ty, etv)
  }
}

impl InstantiationQueuerDeprecated {
  pub fn instantiation_queuer_deprecated_instantiation_queuer_deprecated(
    scope: NonNull<Scope>,
    location: &Location,
    solver: *mut ConstraintSolver,
  ) -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("InstantiationQueuer"), true),
      solver,
      scope,
      location: *location,
    }
  }
}
