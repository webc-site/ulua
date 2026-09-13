//! @interface-stub
use ulua_analysis::type_aliases::type_id::TypeId;

use crate::records::visit_count_tracker::VisitCountTracker;

impl VisitCountTracker {
  pub fn operator_call<T>(&mut self, ty: TypeId, _t: &T) -> bool {
    self.visit_type_id(ty)
  }
}
