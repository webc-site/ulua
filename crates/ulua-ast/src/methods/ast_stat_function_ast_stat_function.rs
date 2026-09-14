use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_function::AstStatFunction, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatFunction {
  pub fn new(location: Location, name: *mut AstExpr, func: *mut AstExprFunction) -> Self {
    Self {
      base: AstStat {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
        has_semicolon: false,
      },
      name,
      func,
    }
  }
}

pub fn ast_stat_function_ast_stat_function(
  location: Location,
  name: *mut AstExpr,
  func: *mut AstExprFunction,
) -> AstStatFunction {
  AstStatFunction::new(location, name, func)
}
