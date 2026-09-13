use alloc::string::String;

use crate::{records::dfg_scope::DfgScope, type_aliases::def_id_def::DefId};

impl DfgScope {
  pub fn lookup_def_id_string(&self, def: DefId, key: &String) -> Option<DefId> {
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
      current = unsafe { scope.parent.as_ref() };
    }
    None
  }
}
