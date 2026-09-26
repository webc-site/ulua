use crate::records::{scope::Scope, scope_registry::resolve_scope};

impl Scope {
  pub fn should_warn_global(&self, name: &str) -> bool {
    let mut current = Some(self);
    while let Some(scope) = current {
      if scope.globals_to_warn.contains_str(name) {
        return true;
      }
      current = scope.parent.and_then(resolve_scope);
    }
    false
  }
}
