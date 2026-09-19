use crate::{records::scope::Scope, type_aliases::scope_ptr_type::ScopePtr};

impl Scope {
  pub fn inherit_assignments(&mut self, child_scope: &ScopePtr) {
    // cpp `lvalueTypes[k] = a`：operator[] 是覆盖写，已存在的键必须刷新
    for (k, a) in child_scope.lvalue_types.iter() {
      *self.lvalue_types.get_or_insert(*k) = *a;
    }
  }
}
