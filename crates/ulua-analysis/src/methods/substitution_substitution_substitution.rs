use alloc::vec::Vec;
use core::ptr::{null, null_mut};

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::records::{
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
  pub fn substitution_new(log_: *const TxnLog, arena: *mut TypeArena) -> Self {
    let mut base = Tarjan {
      type_to_index: DenseHashMap::new(null()),
      pack_to_index: DenseHashMap::new(null()),
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
      arena: null_mut(),
      new_types: DenseHashMap::new(null()),
      new_packs: DenseHashMap::new(null()),
      replaced_types: DenseHashSet::new(null()),
      replaced_type_packs: DenseHashSet::new(null()),
      no_traverse_types: DenseHashSet::new(null()),
      no_traverse_type_packs: DenseHashSet::new(null()),
    };
    this.substitution_txn_log_type_arena(log_, arena);
    this
  }
}
