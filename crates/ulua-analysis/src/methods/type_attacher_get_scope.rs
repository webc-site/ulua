use ulua_ast::records::location::Location;

use crate::{records::type_attacher::TypeAttacher, type_aliases::scope_ptr_type::ScopePtr};

impl TypeAttacher {
  pub fn get_scope(&mut self, loc: &Location) -> ScopePtr {
    let module = unsafe { &*self.module };
    let mut scope_location: Option<Location> = None;
    let mut scope: Option<ScopePtr> = None;

    for (s_loc, s_scope) in &module.scopes {
      if s_loc.encloses(loc)
        && (scope.is_none() || scope_location.as_ref().unwrap().encloses(s_loc))
      {
        scope_location = Some(*s_loc);
        scope = Some(s_scope.clone());
      }
    }

    // The C++ code returns nullptr if no scope is found, but the Rust interface
    // returns a non-optional Arc. In practice there is always at least one scope
    // (the global scope) that encloses any location, so unwrap is safe.
    scope.expect("no enclosing scope found")
  }
}
