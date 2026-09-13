use crate::records::{binding::Binding, scope::Scope};

impl Scope {
  pub fn linear_search_for_binding(
    &self,
    name: &str,
    traverse_scope_chain: bool,
  ) -> Option<Binding> {
    let mut scope: Option<&Scope> = Some(self);

    while let Some(current_scope) = scope {
      for (symbol, binding) in &current_scope.bindings {
        if symbol.name() == name {
          return Some(binding.clone());
        }
      }

      scope = current_scope.parent.as_ref().map(|p| p.as_ref());

      if !traverse_scope_chain {
        break;
      }
    }

    None
  }
}
