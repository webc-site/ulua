//! `bool TypeSearcher::visit(TypeId ty, const FunctionType& ft) override`
//! (`Analysis/src/ConstraintSolver.cpp:837-846`, hand-ported faithfully).
//!
//! The C++ `TypeSearcher : TypeVisitor` overrides are dispatched through the
//! visitor traversal machinery. This file additionally wires `TypeSearcher`
//! into `GenericTypeVisitorTrait` (the same pattern as `InstanceCollector` in
//! `reduce_type_functions`) so that `traverse(...)` can call back into the
//! `visit` overrides — exactly mirroring the C++ override set:
//!   - `visit(TypeId, FunctionType&)` (this file)
//!   - `visit(TypeId)` (`type_searcher_visit_constraint_solver.rs`)
//!   - `visit(TypeId, const ExternType&)` (`type_searcher_visit_constraint_solver_alt_c.rs`)

use core::ffi::c_void;
use std::collections::HashSet;

use crate::{
  methods::generic_type_visitor_traverse_visit_type_alt_b::traverse_type_pack_id,
  records::{
    extern_type::ExternType,
    function_type::FunctionType,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    type_searcher::TypeSearcher,
  },
  type_aliases::type_id::TypeId,
};
impl TypeSearcher {
  pub fn visit_type_id_function_type(&mut self, _ty: TypeId, ft: &FunctionType) -> bool {
    self.flip();
    traverse_type_pack_id(self, ft.arg_types);

    self.flip();
    traverse_type_pack_id(self, ft.ret_types);

    false
  }
}

impl GenericTypeVisitorTrait for TypeSearcher {
  type Seen = HashSet<*mut c_void>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    TypeSearcher::visit_type_id(self, ty)
  }

  fn visit_type_id_function_type(&mut self, ty: TypeId, ftv: &FunctionType) -> bool {
    TypeSearcher::visit_type_id_function_type(self, ty, ftv)
  }

  fn visit_type_id_extern_type(&mut self, ty: TypeId, etv: &ExternType) -> bool {
    TypeSearcher::visit_type_id_extern_type(self, ty, etv)
  }
}
