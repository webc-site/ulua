use crate::{
  records::{dfg_scope::DfgScope, symbol::Symbol},
  type_aliases::def_id_def::DefId,
};

impl DfgScope {
  pub fn lookup_symbol(&self, symbol: Symbol) -> Option<DefId> {
    let mut current = self as *const DfgScope;
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
}
