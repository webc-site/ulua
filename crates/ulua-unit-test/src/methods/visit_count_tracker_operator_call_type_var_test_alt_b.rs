//! @interface-stub
use ulua_analysis::type_aliases::type_pack_id::TypePackId;

use crate::records::visit_count_tracker::VisitCountTracker;

impl VisitCountTracker {
  pub fn operator_call_2<T>(&mut self, tp: TypePackId, _t: &T) -> bool {
    self.visit_type_pack_id(tp)
  }
}
