use crate::{
  records::{
    ast_expr::AstExpr, ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock,
    ast_stat_if::AstStatIf, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatIf {
  pub fn new(
    location: Location,
    condition: *mut AstExpr,
    thenbody: *mut AstStatBlock,
    elsebody: *mut AstStat,
    then_location: Option<Location>,
    else_location: Option<Location>,
  ) -> Self {
    Self {
      base: AstStat {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
        has_semicolon: false,
      },
      condition,
      thenbody,
      elsebody,
      then_location,
      else_location,
    }
  }
}
