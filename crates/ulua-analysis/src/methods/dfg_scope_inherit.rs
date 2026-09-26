use alloc::{string::String, vec::Vec};

use crate::{
  records::{dfg_scope::DfgScope, symbol::Symbol},
  type_aliases::def_id_def::DefId,
};
impl DfgScope {
  /// # Safety
  /// `child_scope` 须指向 `DataFlowGraphBuilder` 在本次建图期间持有的存活 `DfgScope`：非空、对齐，地址
  /// 随 bump 分配稳定不移动；本函数只读取 `child_scope` 的 bindings/props 并写入 `self`，调用方单线程独占，函数返回后指针仍由 builder 持有。
  /// 对应 C++ `void DfgScope::inherit(const DfgScope* childScope)` (`cpp/Analysis/src/DataFlowGraph.cpp:139`)。
  pub unsafe fn inherit(&mut self, child_scope: *const DfgScope) {
    // C++:
    //   for (const auto& [k, a] : childScope->bindings)
    //       if (lookup(k)) bindings[k] = a;
    //   for (const auto& [k1, a1] : childScope->props)
    //       for (const auto& [k2, a2] : a1)
    //           props[k1][k2] = a2;
    unsafe {
      let child_bindings: Vec<(Symbol, DefId)> = (*child_scope)
        .bindings
        .iter()
        .map(|(k, a)| (k.clone(), *a))
        .collect();
      for (k, a) in child_bindings {
        if self.lookup_symbol(k.clone()).is_some() {
          *self.bindings.get_or_insert(k) = a;
        }
      }

      let child_props: Vec<(DefId, Vec<(String, DefId)>)> = (*child_scope)
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
}
