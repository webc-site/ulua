use crate::{records::type_ids::TypeIds, type_aliases::type_id::TypeId};

impl TypeIds {
  pub fn front(&self) -> TypeId {
    self.order[0]
  }
}
