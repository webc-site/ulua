use crate::{
  records::{
    ast_node::AstNode, ast_stat::AstStat, ast_stat_continue::AstStatContinue, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatContinue {
  pub fn new(location: Location) -> Self {
    Self {
      base: AstStat {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
        has_semicolon: false,
      },
    }
  }
}
