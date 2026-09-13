use alloc::string::String;

use ulua_ast::records::location::Location;

use crate::records::scope::Scope;

impl Scope {
  pub fn is_invalid_type_alias(&self, name: &String) -> Option<Location> {
    let mut scope: Option<&Scope> = Some(self);
    while let Some(current_scope) = scope {
      if let Some(loc) = current_scope.invalid_type_aliases.find(name) {
        return Some(*loc);
      }

      scope = current_scope.parent.as_ref().map(|scoped| scoped.as_ref());
    }
    None
  }
}
