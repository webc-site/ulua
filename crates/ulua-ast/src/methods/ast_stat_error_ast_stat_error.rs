use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_error::AstStatError, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatError {
  pub fn new(
    location: Location,
    expressions: AstArray<*mut AstExpr>,
    statements: AstArray<*mut AstStat>,
    message_index: u32,
  ) -> Self {
    AstStatError {
      base: AstStat {
        base: AstNode {
          class_index: Self::CLASS_INDEX,
          location,
        },
        has_semicolon: false,
      },
      expressions,
      statements,
      message_index,
    }
  }
}

pub fn ast_stat_error_ast_stat_error(
  location: Location,
  expressions: AstArray<*mut AstExpr>,
  statements: AstArray<*mut AstStat>,
  message_index: u32,
) -> AstStatError {
  AstStatError::new(location, expressions, statements, message_index)
}
