use ulua_analysis::type_aliases::type_pack_id::TypePackId;

use crate::records::visit_count_tracker::VisitCountTracker;

impl VisitCountTracker {
  pub fn cycle_type_pack_id(&mut self, _tp: TypePackId) {
    // The C++ method body is empty.
  }
}
