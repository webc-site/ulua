use crate::{
  records::{
    ast_expr::AstExpr, ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock,
    ast_stat_while::AstStatWhile, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatWhile {
  pub fn new(
    location: Location,
    condition: *mut AstExpr,
    body: *mut AstStatBlock,
    has_do: bool,
    do_location: Location,
  ) -> Self {
    Self {
      base: AstStat {
        base: AstNode {
          class_index: Self::CLASS_INDEX,
          location,
        },
        has_semicolon: false,
      },
      condition,
      body,
      has_do,
      do_location,
    }
  }
}

pub fn ast_stat_while_ast_stat_while(
  location: Location,
  condition: *mut AstExpr,
  body: *mut AstStatBlock,
  has_do: bool,
  do_location: Location,
) -> AstStatWhile {
  AstStatWhile::new(location, condition, body, has_do, do_location)
}
