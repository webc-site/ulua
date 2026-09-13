use crate::{
  records::{
    ast_expr::AstExpr, ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock,
    ast_stat_repeat::AstStatRepeat, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatRepeat {
  pub fn new(
    location: Location,
    condition: *mut AstExpr,
    body: *mut AstStatBlock,
    deprecated_has_until: bool,
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
      deprecated_has_until,
    }
  }
}

pub fn ast_stat_repeat_ast_stat_repeat(
  location: Location,
  condition: *mut AstExpr,
  body: *mut AstStatBlock,
  deprecated_has_until: bool,
) -> AstStatRepeat {
  AstStatRepeat::new(location, condition, body, deprecated_has_until)
}
