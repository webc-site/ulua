//! `dfg_scope` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{string::String, vec::Vec};

use crate::{
  records::{
    arena_handle::{alias_opt, alias_ref},
    dfg_scope::DfgScope,
    symbol::Symbol,
  },
  type_aliases::def_id_def::DefId,
};

impl DfgScope {
  /// 对应 C++ `void DfgScope::inherit(const DfgScope* childScope)` (`cpp/Analysis/src/DataFlowGraph.cpp:139`)。
  /// 接收安全引用 `&DfgScope`，消除内部 unsafe 块与裸指针入参。
  pub fn inherit(&mut self, child_scope: &DfgScope) {
    let child_bindings: Vec<(Symbol, DefId)> = child_scope
      .bindings
      .iter()
      .map(|(k, a)| (k.clone(), *a))
      .collect();
    for (k, a) in child_bindings {
      if self.lookup_symbol(k.clone()).is_some() {
        *self.bindings.get_or_insert(k) = a;
      }
    }

    let child_props: Vec<(DefId, Vec<(String, DefId)>)> = child_scope
      .props
      .iter()
      .map(|(k1, a1)| (*k1, a1.iter().map(|(k2, a2)| (k2.clone(), *a2)).collect()))
      .collect();
    for (k1, a1) in child_props {
      let entry = self.props.get_or_insert(k1);
      for (k2, a2) in a1 {
        entry.insert(k2, a2);
      }
    }
  }
}

impl DfgScope {
  pub fn lookup_symbol(&self, symbol: Symbol) -> Option<DefId> {
    let mut current = self as *const DfgScope;
    while !current.is_null() {
      if let Some(def) = alias_ref(current).bindings.find(&symbol) {
        return Some(*def);
      }

      current = alias_ref(current).parent as *const DfgScope;
    }

    None
  }

  pub fn lookup_def_id_string(&self, def: DefId, key: &str) -> Option<DefId> {
    // C++: for (current = this; current; current = current->parent)
    //          if (auto props = current->props.find(def))
    //              if (auto it = props->find(key); it != props->end())
    //                  return NotNull{it->second};
    let mut current: Option<&DfgScope> = Some(self);
    while let Some(scope) = current {
      if let Some(props) = scope.props.find(&def)
        && let Some(value) = props.get(key)
      {
        return Some(*value);
      }
      current = alias_opt(scope.parent);
    }
    None
  }
}
