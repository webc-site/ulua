use core::{cmp, ffi::c_void};

use ulua_ast::{
  records::{
    ast_stat::AstStat, ast_stat_assign::AstStatAssign, ast_stat_expr::AstStatExpr,
    ast_stat_local::AstStatLocal, location::Location, position::Position,
  },
  rtti::ast_node_is,
};

use crate::records::lint_implicit_return::LintImplicitReturn;
pub fn lint_implicit_return_get_end_location(
  _this: &mut LintImplicitReturn,
  node: *const c_void,
) -> Location {
  let node = node as *const AstStat;
  let node_ref = unsafe { &*node };
  let loc = node_ref.base.location;

  if ast_node_is::<AstStatExpr>(&node_ref.base)
    || ast_node_is::<AstStatAssign>(&node_ref.base)
    || ast_node_is::<AstStatLocal>(&node_ref.base)
  {
    return loc;
  }

  if loc.begin.line == loc.end.line {
    return loc;
  }

  let column = cmp::max(0, loc.end.column as i32 - 3) as u32;
  Location::new(Position::new(loc.end.line, column), loc.end)
}
