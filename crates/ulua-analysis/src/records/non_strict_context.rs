use alloc::collections::BTreeMap;

use crate::type_aliases::{def_id_def::DefId, type_id::TypeId};

#[derive(Debug, Clone, Default)]
pub struct NonStrictContext {
  pub context: BTreeMap<DefId, TypeId>,
}

impl NonStrictContext {
  pub fn find(&self, d: DefId) -> Option<TypeId> {
    self.context.get(&d).copied()
  }
}
