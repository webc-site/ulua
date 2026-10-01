use alloc::vec::Vec;

use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};
#[derive(Debug, Clone)]
pub struct TypePack {
  pub(crate) head: Vec<TypeId>,
  pub(crate) tail: Option<TypePackId>,
}

impl TypePack {
  #[inline]
  pub fn new(head: Vec<TypeId>, tail: Option<TypePackId>) -> Self {
    Self { head, tail }
  }

  #[inline]
  pub const fn empty() -> Self {
    Self {
      head: Vec::new(),
      tail: None,
    }
  }

  #[inline]
  pub fn single(ty: TypeId) -> Self {
    Self {
      head: alloc::vec![ty],
      tail: None,
    }
  }

  #[inline]
  pub fn from_vec(head: Vec<TypeId>) -> Self {
    Self { head, tail: None }
  }
}
