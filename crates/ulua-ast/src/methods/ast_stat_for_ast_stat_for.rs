use crate::{
  records::{
    ast_expr::AstExpr, ast_local::AstLocal, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_block::AstStatBlock, ast_stat_for::AstStatFor, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatFor {
  pub fn new(
    location: Location,
    var: *mut AstLocal,
    from: *mut AstExpr,
    to: *mut AstExpr,
    step: *mut AstExpr,
    body: *mut AstStatBlock,
    has_do: bool,
    do_location: Location,
  ) -> Self {
    Self {
      base: AstStat {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
        has_semicolon: false,
      },
      var,
      from,
      to,
      step,
      body,
      has_do,
      do_location,
    }
  }
}

pub fn ast_stat_for_ast_stat_for(
  location: Location,
  var: *mut AstLocal,
  from: *mut AstExpr,
  to: *mut AstExpr,
  step: *mut AstExpr,
  body: *mut AstStatBlock,
  has_do: bool,
  do_location: Location,
) -> AstStatFor {
  AstStatFor::new(location, var, from, to, step, body, has_do, do_location)
}
