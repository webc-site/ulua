use ulua_ast::records::location::Location;

use crate::records::type_checker::TypeChecker;

impl TypeChecker {
  pub fn ice_string_location(&mut self, message: &str, location: &Location) {
    self.ice_string(message);
    let _ = location;
  }
}
