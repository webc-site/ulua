use ulua_ast::records::location::Location;

use crate::records::{
  compile_error::{CompileError, ERR_EXCEEDED_CONSTANT_LIMIT},
  compiler::Compiler,
};

impl Compiler {
  pub fn check_constant(&mut self, constant: i32, location: &Location) {
    if constant < 0 {
      CompileError::raise(
        location,
        core::format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
      );
    }
  }
}
