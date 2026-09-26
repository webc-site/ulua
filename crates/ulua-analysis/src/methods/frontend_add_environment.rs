use alloc::string::String;

use crate::{
  records::{frontend::Frontend, scope::Scope, scope_registry::register_scope},
  type_aliases::scope_ptr_type::ScopePtr,
};
impl Frontend {
  pub fn add_environment(&mut self, environment_name: String) -> ScopePtr {
    if let Some(scope) = self.environments.get(&environment_name) {
      return scope.clone();
    }
    let scope = ScopePtr::new(Scope::new(&self.globals.global_scope, 0));
    register_scope(&scope);
    self.environments.insert(environment_name, scope.clone());
    scope
  }
}
