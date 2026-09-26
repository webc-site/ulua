use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{arena_handle::Handle, tarjan::Tarjan, type_arena::TypeArena},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct Substitution {
  pub(crate) base: Tarjan,
  // cpp `TypeInfer.cpp:214` 以 nullptr 构造 `reusableInstantiation` 的占位 arena，
  // 每次 check 前经 `resetState` 接线（TypeInfer.cpp:5311）——构造期可空，使用前必非空。
  pub(crate) arena: Option<Handle<TypeArena>>,
  pub(crate) new_types: DenseHashMap<TypeId, TypeId>,
  pub(crate) new_packs: DenseHashMap<TypePackId, TypePackId>,
  pub(crate) replaced_types: DenseHashSet<TypeId>,
  pub(crate) replaced_type_packs: DenseHashSet<TypePackId>,
  pub(crate) no_traverse_types: DenseHashSet<TypeId>,
  pub(crate) no_traverse_type_packs: DenseHashSet<TypePackId>,
}
