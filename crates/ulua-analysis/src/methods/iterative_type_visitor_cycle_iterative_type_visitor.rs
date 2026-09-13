use crate::{records::iterative_type_visitor::IterativeTypeVisitor, type_aliases::type_id::TypeId};

impl IterativeTypeVisitor {
  pub fn cycle_type_id(&mut self, _ty: TypeId) {
    // Empty implementation per source: void IterativeTypeVisitor::cycle(TypeId) {}
  }
}
