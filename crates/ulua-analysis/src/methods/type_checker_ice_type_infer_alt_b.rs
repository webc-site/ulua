use ulua_ast::records::location::Location;

use crate::records::type_checker::TypeChecker;
impl TypeChecker {
  pub fn ice_string(&mut self, message: &str) {
    self.ice_string_location(message, &Location::default());
  }
}
