use ulua_ast::records::location::Location;

use crate::records::scope::Scope;

impl Scope {
  pub fn find_narrowest_scope_containing(&mut self, location: Location) -> *mut Scope {
    let mut best_scope = self as *mut Scope;

    loop {
      let mut did_narrow = false;
      let children = unsafe { &(*best_scope).children };

      for &scope in children {
        if unsafe { (*scope).location.encloses(&location) } {
          best_scope = scope;
          did_narrow = true;
          break;
        }
      }

      if !did_narrow || unsafe { (*best_scope).children.is_empty() } {
        break;
      }
    }

    best_scope
  }
}
