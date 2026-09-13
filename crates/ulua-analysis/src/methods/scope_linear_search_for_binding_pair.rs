use crate::records::{binding::Binding, scope::Scope, symbol::Symbol};

impl Scope {
  pub fn linear_search_for_binding_pair(
    &self,
    name: &str,
    traverse_scope_chain: bool,
  ) -> Option<(Symbol, Binding)> {
    let mut scope: Option<&Scope> = Some(self);

    while let Some(current_scope) = scope {
      for (symbol, binding) in &current_scope.bindings {
        if symbol.name() == name {
          return Some((symbol.clone(), binding.clone()));
        }
      }

      if !traverse_scope_chain {
        break;
      }

      scope = current_scope.parent.as_ref().map(|p| p.as_ref());
    }

    None
  }
}
