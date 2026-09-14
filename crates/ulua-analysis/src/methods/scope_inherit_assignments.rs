use crate::{records::scope::Scope, type_aliases::scope_ptr_type::ScopePtr};

impl Scope {
  pub fn inherit_assignments(&mut self, child_scope: &ScopePtr) {
    for (k, a) in { &child_scope.lvalue_types }.iter() {
      self.lvalue_types.try_insert(*k, *a);
    }
  }
}
