use alloc::vec::Vec;
use core::ptr::null;

use ulua_common::{
  macros::luau_assert::LUAU_ASSERT,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::records::{
  arena_handle::Handle,
  arena_id::ArenaId,
  substitution::Substitution,
  tarjan::{SubstitutionVtable, Tarjan},
  txn_log::TxnLog,
  type_arena::TypeArena,
};

impl Substitution {
  /// C++ `Substitution::Substitution(const TxnLog* log_, TypeArena* arena)`
  /// (`Substitution.cpp`). Builds a fresh `Substitution` value with empty
  /// containers and the given log/arena; the C++ base `Tarjan()` constructor
  /// reserves space for its worklists, mirrored by `Tarjan::tarjan`.
  pub fn substitution_new(log_: *const TxnLog, arena: Option<Handle<TypeArena>>) -> Self {
    let mut base = Tarjan {
      type_to_index: DenseHashMap::default(),
      pack_to_index: DenseHashMap::default(),
      nodes: Vec::new(),
      stack: Vec::new(),
      child_count: 0,
      child_limit: 0,
      log: null(),
      edges_ty: Vec::new(),
      edges_tp: Vec::new(),
      worklist: Vec::new(),
      vtable: SubstitutionVtable::null(),
    };
    base.tarjan();

    let mut this = Substitution {
      base,
      arena,
      new_types: DenseHashMap::default(),
      new_packs: DenseHashMap::default(),
      replaced_types: DenseHashSet::default(),
      replaced_type_packs: DenseHashSet::default(),
      no_traverse_types: DenseHashSet::default(),
      no_traverse_type_packs: DenseHashSet::default(),
    };
    this.substitution_txn_log_type_arena(log_, arena);
    this
  }

  pub fn substitution_txn_log_type_arena(
    &mut self,
    log_: *const TxnLog,
    arena: Option<Handle<TypeArena>>,
  ) {
    self.arena = arena;
    self.base.log = log_;
    LUAU_ASSERT!(!log_.is_null());
  }

  /// 已接线的 arena 可变视图：遍历期（`reset_state` 之后）恒非空；
  /// 构造占位期的 null 属契约违例，确定性 panic 而非 UB。
  pub(crate) fn wired_arena_mut(&self) -> &mut TypeArena {
    self
      .arena
      .expect("Substitution.arena 使用前必须已由 reset_state 接线")
      .get_mut()
  }

  /// 已接线 arena 的身份值，供与节点 `owning_arena` 做归属比较。
  pub(crate) fn wired_arena_id(&self) -> ArenaId {
    self.wired_arena_handle().get().arena_id
  }

  /// 已接线 arena 的句柄形态（判空即 panic，与 `wired_arena_id` 同一契约），
  /// 供以 `Handle<TypeArena>` 为形参的下游接口直传，避免句柄↔裸指针往返。
  pub(crate) fn wired_arena_handle(&self) -> Handle<TypeArena> {
    self
      .arena
      .expect("Substitution.arena 使用前必须已由 reset_state 接线")
  }
}
