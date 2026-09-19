use alloc::string::String;

use crate::records::scope::Scope;

impl Scope {
  pub fn should_warn_global(&self, name: String) -> bool {
    let mut current = Some(self);
    while let Some(scope) = current {
      if scope.globals_to_warn.contains(&name) {
        return true;
      }
      current = scope.parent.as_deref();
    }
    false
  }
}
