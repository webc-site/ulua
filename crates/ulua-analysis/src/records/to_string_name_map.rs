use alloc::string::String;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};

#[derive(Debug, Clone)]
pub struct ToStringNameMap {
  pub types: DenseHashMap<TypeId, String>,
  pub type_packs: DenseHashMap<TypePackId, String>,
}
