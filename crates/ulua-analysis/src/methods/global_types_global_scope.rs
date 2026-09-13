use crate::{records::global_types::GlobalTypes, type_aliases::scope_ptr_type::ScopePtr};

impl GlobalTypes {
  pub fn global_scope(&self) -> ScopePtr {
    self.global_scope.clone()
  }
}
