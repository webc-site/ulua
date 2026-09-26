use alloc::vec::IntoIter;

use crate::{records::type_ids::TypeIds, type_aliases::type_id::TypeId};

impl TypeIds {
  pub fn begin(&self) -> IntoIter<TypeId> {
    self.order.clone().into_iter()
  }
}
