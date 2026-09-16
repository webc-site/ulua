use alloc::vec::Vec;

use crate::{records::type_pack::TypePack, type_aliases::type_id::TypeId};

impl TypePack {
  pub fn head(&self) -> &Vec<TypeId> {
    &self.head
  }
}
