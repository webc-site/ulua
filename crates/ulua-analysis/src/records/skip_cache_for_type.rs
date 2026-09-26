use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{records::arena_id::ArenaId, type_aliases::type_id::TypeId};
#[derive(Debug, Clone)]
pub struct SkipCacheForType {
  pub skip_cache_for_type: *const DenseHashMap<TypeId, bool>,
  /// 本遍历判归属用的 arena 身份（cpp `TypeArena* arena`）。
  pub type_arena_id: ArenaId,
  pub result: bool,
}
