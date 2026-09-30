//! Source: `Analysis/include/Luau/TypeArena.h` (TypeArena.h:15-27, hand-ported)

use alloc::string::String;
use core::ptr::NonNull;

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

  /// §2：cpp `Module* owningModule = nullptr`（「owning module, if any」，可空回指）。
  /// 这是 arena 对宿主 `Module` 的自引用回指——`Module` 按值持有本 `TypeArena`
  /// （`interface_types`/`internal_types`），生命周期回环使借用检查无法表达，故按
  /// 「被 `TypeArena` 持有的回指」建模为 `Option<NonNull<Module>>`：`None` 即无宿主，
  /// `Some` 编码非空。全仓该字段仅由 `frontend`/`type_checker` 在 `Arc<Module>` 写穿
  /// 时注入自指针、无任何读取点（从不解引用），故无需 chokepoint，裸指针语义不外渗。
  pub owning_module: Option<NonNull<Module>>,

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
      owning_module: None,
      collect_singleton_stats: false,
      bool_singletons_minted: 0,
      str_singletons_minted: 0,
      unique_str_singletons_minted: DenseHashSet::default(),
    }
  }
}
