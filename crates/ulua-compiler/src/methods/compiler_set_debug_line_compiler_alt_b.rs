use ulua_ast::records::location::Location;

use crate::records::compiler::Compiler;

impl Compiler {
  pub fn set_debug_line_location(&mut self, location: &Location) {
    if self.options.debug_level >= 1 {
      unsafe {
        (*self.bytecode).set_debug_line((location.begin.line + 1) as i32);
      }
    }
  }
}
