use std::mem;

use crate::{records::type_ids::TypeIds, type_aliases::type_id::TypeId};
impl TypeIds {
  pub fn take(&mut self) -> Vec<TypeId> {
    self.hash = 0;
    self.types.clear();
    mem::take(&mut self.order)
  }
}
