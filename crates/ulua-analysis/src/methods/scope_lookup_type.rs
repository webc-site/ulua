use crate::records::{scope::Scope, scope_registry::resolve_scope, type_fun::TypeFun};

impl Scope {
  pub fn lookup_type(&self, name: &str) -> Option<TypeFun> {
    let mut current_scope: &Scope = self;
    loop {
      if let Some(type_fun) = current_scope.exported_type_bindings.get(name) {
        return Some(type_fun.clone());
      }

      if let Some(type_fun) = current_scope.private_type_bindings.get(name) {
        return Some(type_fun.clone());
      }

      current_scope = current_scope.parent.and_then(resolve_scope)?;
    }
  }
}
