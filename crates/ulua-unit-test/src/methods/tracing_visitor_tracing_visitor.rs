use alloc::vec::Vec;

use ulua_analysis::{
  records::iterative_type_visitor::{IterativeTypeVisitor, IterativeTypeVisitorTrait},
  type_aliases::type_id::TypeId,
};

use crate::records::tracing_visitor::TracingVisitor;
impl TracingVisitor {
  pub fn new(visit_once: bool, skip_bound_types: bool) -> Self {
    let mut base = IterativeTypeVisitor::default();
    base.iterative_type_visitor_string_bool_bool("TracingVisitor", visit_once, skip_bound_types);

    Self {
      base,
      trace: Vec::new(),
      cycles: Vec::new(),
    }
  }

  pub fn run_type_id(&mut self, ty: TypeId) {
    IterativeTypeVisitorTrait::run_type_id(self, ty);
  }
}

impl IterativeTypeVisitorTrait for TracingVisitor {
  fn visitor_base(&mut self) -> &mut IterativeTypeVisitor {
    &mut self.base
  }

  fn cycle_type_id(&mut self, ty: TypeId) {
    TracingVisitor::cycle_type_id(self, ty);
  }

  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    TracingVisitor::visit_type_id(self, ty)
  }
}
