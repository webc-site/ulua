use crate::{
  records::{dfg_scope::DfgScope, symbol::Symbol},
  type_aliases::def_id_def::DefId,
};

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
