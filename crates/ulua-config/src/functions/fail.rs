use ulua_ast::records::lexer::Lexer;
use ulua_common::functions::format::format;

use crate::type_aliases::error::Error;

pub(crate) fn fail(lexer: &Lexer, message: &str) -> Error {
  let cur = lexer.current();
  Some(format(format_args!(
    "Expected {} at line {}, got {} instead",
    message,
    cur.location.begin.line + 1,
    cur
  )))
}
