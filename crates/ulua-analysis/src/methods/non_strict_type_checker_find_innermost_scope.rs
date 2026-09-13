use ulua_ast::records::location::Location;

use crate::records::{non_strict_type_checker::NonStrictTypeChecker, scope::Scope};

impl NonStrictTypeChecker {
  pub fn find_innermost_scope(&self, location: Location) -> *mut Scope {
    let mut best_scope: *mut Scope =
      unsafe { (*self.module).get_module_scope().as_ref() as *const Scope as *mut Scope };

    let mut did_narrow;
    loop {
      did_narrow = false;
      unsafe {
        for &scope in (*best_scope).children.iter() {
          if (*scope).location.encloses(&location) {
            best_scope = scope;
            did_narrow = true;
            break;
          }
        }

        if !(did_narrow && !(*best_scope).children.is_empty()) {
          break;
        }
      }
    }

    best_scope
  }
}
