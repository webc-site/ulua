use ulua_ast::records::{ast_expr_function::AstExprFunction, location::Location};

pub fn get_end_location(function: &AstExprFunction) -> Location {
  let mut loc = function.base.base.location;
  if loc.begin.line != loc.end.line {
    let mut begin = loc.end;
    begin.column = begin.column.saturating_sub(3);
    loc = Location::with_length(begin, 3);
  }

  loc
}
