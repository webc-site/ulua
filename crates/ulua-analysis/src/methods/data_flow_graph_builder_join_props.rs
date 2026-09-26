use alloc::{collections::BTreeMap, string::String, vec::Vec};

use crate::{
  records::{data_flow_graph_builder::DataFlowGraphBuilder, dfg_scope::DfgScope},
  type_aliases::def_id_def::DefId,
};

impl DataFlowGraphBuilder {
  /// # Safety
  /// `result` 须指向 `DataFlowGraphBuilder` 在本次建图期间持有的存活 `DfgScope`：非空、对齐，地址
  /// 随 bump 分配稳定不移动；本函数从 a/b 读取、写入 result，调用方单线程独占，函数返回后指针仍由 builder 持有。
  /// 对应 C++ `void DataFlowGraphBuilder::joinProps(DfgScope* result, const DfgScope&, const DfgScope&)` (`cpp/Analysis/src/DataFlowGraph.cpp:254`)。
  /// `void DataFlowGraphBuilder::joinProps(DfgScope* result, const DfgScope& a, const DfgScope& b)`.
  /// Reference: `DataFlowGraph.cpp:246-294`.
  ///
  /// Borrow-safety note: the `while`-loop visitor calls `join(scope, scope, whileScope)`,
  /// so `result` and `a` are the **same** `DfgScope`. The faithful C++ mutates
  /// `result->props` while iterating `a->props` (and reads `scope->props` inside
  /// `joinProps`' lambda while holding a reference into it) — in Rust that is `&`/`&mut`
  /// aliasing UB: even without a rehash the optimizer assumes the `&mut` is unique and
  /// miscompiles, corrupting the `props` map's `Vec` Header (observed as a later
  /// `find` indexing an empty `data`). We therefore **snapshot** `a`'s and `b`'s props
  /// into owned values first, and have `phinodify` build each merged entry in a local
  /// `BTreeMap` before writing it back — exactly the pattern `DfgScope::inherit` uses.
  /// The observable result is identical to the C++ (`lookup_def_id_string` is only
  /// consulted when the in-progress entry has no value for the key, so a deferred
  /// write-back cannot change its answer).
  pub unsafe fn join_props(&mut self, result: *mut DfgScope, a: &DfgScope, b: &DfgScope) {
    let def_arena = self.def_arena;

    // Owned snapshots so no borrow of `a`/`b`/`result` props is held across the
    // mutations of `result.props` below (result may alias `a`).
    let a_props: Vec<(DefId, BTreeMap<String, DefId>)> =
      a.props.iter().map(|(k, v)| (*k, v.clone())).collect();
    let b_props: Vec<(DefId, BTreeMap<String, DefId>)> =
      b.props.iter().map(|(k, v)| (*k, v.clone())).collect();
    let b_lookup = |def: DefId| -> Option<&BTreeMap<String, DefId>> {
      b_props.iter().find(|(k, _)| *k == def).map(|(_, v)| v)
    };

    // C++ lambda `phinodify`: merges per-key defs of `a`/`b` into `scope->props[parent]`.
    // Builds the merged entry in a local map, then writes it back — never holds a
    // `&mut` into `scope.props` across the `lookup_def_id_string` read of `scope.props`.
    let phinodify = |scope: *mut DfgScope,
                     a_props: &BTreeMap<String, DefId>,
                     b_props: &BTreeMap<String, DefId>,
                     parent: DefId| unsafe {
      // Safety: `scope` 即传入的 `result`，指向 DFG arena 中存活的 `DfgScope`；`def_arena` 为
      // 非空 bump arena，其 Def/phi 节点地址永不移动。`a_props`/`b_props` 是已克隆的拥有快照，
      // 对 `a`/`b` 的借用早已结束，故即便 `result` 与 `a` 同一（join(scope,scope,whileScope)），
      // 对 `(*scope).props` 的写入也不与任何在持有的 `&` 别名冲突；`lookup_def_id_string` 只读。
      // 全程单线程串行。
      let mut p: BTreeMap<String, DefId> =
        (*scope).props.find(&parent).cloned().unwrap_or_default();

      for (k, def_a) in a_props.iter() {
        let merged = if let Some(it) = b_props.get(k) {
          def_arena.get_mut().phi_def_id_def_id(*it, *def_a)
        } else if let Some(it) = p.get(k).copied() {
          def_arena.get_mut().phi_def_id_def_id(it, *def_a)
        } else if let Some(def2) = (*scope).lookup_def_id_string(parent, k) {
          def_arena.get_mut().phi_def_id_def_id(def2, *def_a)
        } else {
          *def_a
        };
        p.insert(k.clone(), merged);
      }

      for (k, def_b) in b_props.iter() {
        if a_props.get(k).is_some() {
          continue;
        }
        let merged = if let Some(it) = p.get(k).copied() {
          def_arena.get_mut().phi_def_id_def_id(it, *def_b)
        } else if let Some(def2) = (*scope).lookup_def_id_string(parent, k) {
          def_arena.get_mut().phi_def_id_def_id(def2, *def_b)
        } else {
          *def_b
        };
        p.insert(k.clone(), merged);
      }

      *(*scope).props.get_or_insert(parent) = p;
    };

    unsafe {
      // Safety: `result` 为 DFG arena 中存活的 `DfgScope`；`a_props`/`b_props` 是上方克隆的拥有
      // 快照（对 `a`/`b` 的借用已结束），`(*result).props.find` 命中值随即 `.clone()` 后才交给
      // phinodify，因而对 `(*result).props` 的插入/写入不与任何在持有的 `&` 别名冲突（即便
      // result 与 a 同一）。单线程串行遍历。
      for (def, a1) in a_props.iter() {
        (*result).props.try_insert(*def, BTreeMap::new());
        if let Some(a2) = b_lookup(*def) {
          phinodify(result, a1, a2, *def);
        } else if let Some(a2) = (*result).props.find(def) {
          let a2 = a2.clone();
          phinodify(result, a1, &a2, *def);
        }
      }

      for (def, a1) in b_props.iter() {
        (*result).props.try_insert(*def, BTreeMap::new());
        if a_props.iter().any(|(k, _)| k == def) {
          continue;
        } else if let Some(a2) = (*result).props.find(def) {
          let a2 = a2.clone();
          phinodify(result, a1, &a2, *def);
        }
      }
    }
  }
}
