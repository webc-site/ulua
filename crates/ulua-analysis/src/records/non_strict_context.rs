use alloc::collections::BTreeMap;

use crate::{records::def::Def, type_aliases::type_id::TypeId};

#[derive(Debug, Clone, Default)]
pub struct NonStrictContext {
  pub context: BTreeMap<*const Def, TypeId>,
}

impl NonStrictContext {
  pub fn find(&self, d: *const Def) -> Option<TypeId> {
    self.context.get(&d).copied()
  }
}
