use alloc::vec::Vec;

use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};
#[derive(Debug, Clone)]
pub struct TypePack {
  pub(crate) head: Vec<TypeId>,
  pub(crate) tail: Option<TypePackId>,
}

impl TypePack {
  pub fn new(head: Vec<TypeId>, tail: Option<TypePackId>) -> Self {
    Self { head, tail }
  }
}
