use ulua_ast::records::location::Location;

use crate::records::unifier::Unifier;
impl Unifier {
  pub fn ice_string(&mut self, message: &str) {
    self.ice_string_location(message, &Location::default());
  }
}
