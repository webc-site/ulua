use ulua_analysis::{records::table_type::TableType, type_aliases::type_id::TypeId};

use crate::records::table_skipping_visitor::TableSkippingVisitor;

impl TableSkippingVisitor {
  pub fn visit_type_id_table_type(&mut self, _ty: TypeId, _tt: &TableType) -> bool {
    false
  }
}
