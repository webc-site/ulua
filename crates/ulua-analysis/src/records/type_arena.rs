//! Source: `Analysis/include/Luau/TypeArena.h` (TypeArena.h:15-27, hand-ported)

use alloc::string::String;
use core::ptr::null_mut;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::{
  arena_id::{ArenaId, next_arena_id},
  module::Module,
  r#type::Type,
  type_pack_var::TypePackVar,
  typed_allocator::TypedAllocator,
};
#[derive(Debug)]
pub struct TypeArena {
  /// 本 arena 的进程级唯一身份，构造时发号、此后不可变；`Type`/`TypePackVar`
  /// 的 `owning_arena` 存其副本做归属比较（对应 cpp `TypeArena* owningArena`
  /// 的指针同一性）。
  pub arena_id: ArenaId,
  pub types: TypedAllocator<Type>,
  pub type_packs: TypedAllocator<TypePackVar>,

  /// Owning module, if any
  pub owning_module: *mut Module,

  pub collect_singleton_stats: bool,
  pub bool_singletons_minted: usize,
  pub str_singletons_minted: usize,
  pub unique_str_singletons_minted: DenseHashSet<Option<String>>,
}

impl Default for TypeArena {
  fn default() -> Self {
    Self {
      arena_id: next_arena_id(),
      types: TypedAllocator::default(),
      type_packs: TypedAllocator::default(),
      owning_module: null_mut(),
      collect_singleton_stats: false,
      bool_singletons_minted: 0,
      str_singletons_minted: 0,
      unique_str_singletons_minted: DenseHashSet::default(),
    }
  }
}
