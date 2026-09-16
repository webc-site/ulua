use ulua_ast::records::location::Location;

use crate::{
  methods::compile_error_raise::compile_error_raise,
  records::{compile_error::ERR_EXCEEDED_CONSTANT_LIMIT, compiler::Compiler},
};

impl Compiler {
  pub fn check_constant(&mut self, constant: i32, location: &Location) {
    if constant < 0 {
      compile_error_raise(
        *location,
        core::format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
      );
    }
  }
}
