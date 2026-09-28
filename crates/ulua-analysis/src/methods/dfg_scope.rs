//! `dfg_scope` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

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

impl DfgScope {
  pub fn lookup_symbol(&self, symbol: Symbol) -> Option<DefId> {
    let mut current = self as *const DfgScope;
    // Safety: current 初值来自 &self（必有效），后续值是 parent 裸指针——子 scope
    // 由 make_child_scope 从 PinnedStorage<DfgScope>（地址不移动）取地址接线，指向
    // builder 拥有的存活 scope 或 null（root，由循环守卫终止）；循环只读
    // bindings/parent，lookup 期间整棵 scope 树保持稳定。
    unsafe {
      while !current.is_null() {
        if let Some(def) = (*current).bindings.find(&symbol) {
          return Some(*def);
        }

        current = (*current).parent as *const DfgScope;
      }
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
      // Safety: scope.parent 在 make_child_scope 构造期接线，非空时指向
      // PinnedStorage<DfgScope> 中地址稳定的存活父 scope（寿命覆盖本次查找），
      // null 使 as_ref 得 None 正常终止循环；仅重建共享借用，只读无别名冲突。
      current = unsafe { scope.parent.as_ref() };
    }
    None
  }
}
