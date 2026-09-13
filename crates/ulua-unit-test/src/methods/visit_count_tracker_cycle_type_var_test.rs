use ulua_analysis::type_aliases::type_id::TypeId;

use crate::records::visit_count_tracker::VisitCountTracker;

impl VisitCountTracker {
  pub fn cycle_type_id(&mut self, _ty: TypeId) {
    // Empty implementation: the C++ method body is empty
  }
}
