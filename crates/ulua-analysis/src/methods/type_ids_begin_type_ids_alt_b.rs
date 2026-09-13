use crate::{records::type_ids::TypeIds, type_aliases::const_iterator::ConstIterator};

impl TypeIds {
  pub fn begin(&self) -> ConstIterator {
    self.order.clone().into_iter()
  }
}
