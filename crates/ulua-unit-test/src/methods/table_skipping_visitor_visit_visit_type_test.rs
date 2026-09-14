use ulua_analysis::{
  functions::to_string_to_string_alt_c::to_string_type_id, type_aliases::type_id::TypeId,
};

use crate::records::table_skipping_visitor::TableSkippingVisitor;

impl TableSkippingVisitor {
  pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
    self.trace.push(to_string_type_id(ty));
    true
  }
}
