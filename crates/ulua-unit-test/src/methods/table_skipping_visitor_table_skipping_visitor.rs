use alloc::vec::Vec;

use ulua_analysis::{
  records::{
    iterative_type_visitor::{IterativeTypeVisitor, IterativeTypeVisitorTrait},
    table_type::TableType,
  },
  type_aliases::type_id::TypeId,
};

use crate::records::table_skipping_visitor::TableSkippingVisitor;
impl TableSkippingVisitor {
  pub fn table_skipping_visitor_table_skipping_visitor() -> Self {
    Self::new()
  }

  pub fn new() -> Self {
    let mut base = IterativeTypeVisitor::default();
    base.iterative_type_visitor_string_bool_bool("TracingVisitor", true, true);

    Self {
      base,
      trace: Vec::new(),
    }
  }

  pub fn run_type_id(&mut self, ty: TypeId) {
    IterativeTypeVisitorTrait::run_type_id(self, ty);
  }
}

impl Default for TableSkippingVisitor {
  fn default() -> Self {
    Self::new()
  }
}

impl IterativeTypeVisitorTrait for TableSkippingVisitor {
  fn visitor_base(&mut self) -> &mut IterativeTypeVisitor {
    &mut self.base
  }

  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    TableSkippingVisitor::visit_type_id(self, ty)
  }

  fn visit_type_id_table_type(&mut self, ty: TypeId, tt: &TableType) -> bool {
    TableSkippingVisitor::visit_type_id_table_type(self, ty, tt)
  }
}
