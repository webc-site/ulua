use crate::{records::blocked_type_finder::BlockedTypeFinder, type_aliases::type_id::TypeId};

impl BlockedTypeFinder {
  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    self.blocked.is_none()
  }
}
