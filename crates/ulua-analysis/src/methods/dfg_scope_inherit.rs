use alloc::{string::String, vec::Vec};

use crate::records::{def::Def, dfg_scope::DfgScope, symbol::Symbol};
impl DfgScope {
  /// # Safety
  /// 调用方须保证 `child_scope` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn inherit(&mut self, child_scope: *const DfgScope) {
    // C++:
    //   for (const auto& [k, a] : childScope->bindings)
    //       if (lookup(k)) bindings[k] = a;
    //   for (const auto& [k1, a1] : childScope->props)
    //       for (const auto& [k2, a2] : a1)
    //           props[k1][k2] = a2;
    unsafe {
      let child_bindings: Vec<(Symbol, *const Def)> = (*child_scope)
        .bindings
        .iter()
        .map(|(k, a)| (k.clone(), *a))
        .collect();
      for (k, a) in child_bindings {
        if self.lookup_symbol(k.clone()).is_some() {
          *self.bindings.get_or_insert(k) = a;
        }
      }

      let child_props: Vec<(*const Def, Vec<(String, *const Def)>)> = (*child_scope)
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
