use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_stat::AstStat, ast_stat_error::AstStatError,
    location::Location,
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
      base: AstStat::new(Self::CLASS_INDEX, location),
      expressions,
      statements,
      message_index,
    }
  }
}
