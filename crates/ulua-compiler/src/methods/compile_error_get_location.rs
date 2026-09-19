use ulua_ast::records::location::Location;

use crate::records::compile_error::CompileError;

impl CompileError {
  pub fn get_location(&self) -> &Location {
    &self.location
  }
}
