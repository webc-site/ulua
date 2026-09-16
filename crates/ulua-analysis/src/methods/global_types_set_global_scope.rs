use crate::{records::global_types::GlobalTypes, type_aliases::scope_ptr_type::ScopePtr};

impl GlobalTypes {
  pub fn set_global_scope(&mut self, global_scope: ScopePtr) {
    self.global_scope = global_scope;
  }
}
