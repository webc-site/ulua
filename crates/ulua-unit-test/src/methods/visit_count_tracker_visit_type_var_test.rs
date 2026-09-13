//! @interface-stub
use ulua_analysis::type_aliases::type_id::TypeId;

use crate::records::visit_count_tracker::VisitCountTracker;

impl VisitCountTracker {
  pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
    self.visit_type(ty)
  }
}
