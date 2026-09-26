use crate::{
  methods::generic_type_visitor_traverse_visit_type::traverse_type_pack_id,
  records::{
    extern_type::ExternType,
    function_type::FunctionType,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    type_searcher::TypeSearcher,
    visit_key::VisitKey,
  },
  type_aliases::{collections::HashSet, type_id::TypeId},
};

impl TypeSearcher {
  pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
    if ty == self.needle {
      self.count += 1;
      self.result |= self.current;
    }

    true
  }
}

// `bool TypeSearcher::visit(TypeId ty, const FunctionType& ft) override`
// (`Analysis/src/ConstraintSolver.cpp:837-846`, hand-ported faithfully).
//
// The C++ `TypeSearcher : TypeVisitor` overrides are dispatched through the
// visitor traversal machinery. This file additionally wires `TypeSearcher`
// into `GenericTypeVisitorTrait` (the same pattern as `InstanceCollector` in
// `reduce_type_functions`) so that `traverse(...)` can call back into the
// `visit` overrides — exactly mirroring the C++ override set:
//   - `visit(TypeId, FunctionType&)` (this file)
//   - `visit(TypeId)` (`type_searcher_visit_constraint_solver.rs`)
//   - `visit(TypeId, const ExternType&)` (`type_searcher_visit_constraint_solver.rs`)
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
  type Seen = HashSet<VisitKey>;

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

impl TypeSearcher {
  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _et: &ExternType) -> bool {
    false
  }
}
