use alloc::vec::Vec;

use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};
#[derive(Debug, Clone, Default)]
pub struct InteriorFreeTypes {
  pub types: Vec<TypeId>,
  pub type_packs: Vec<TypePackId>,
}
