use crate::{
  records::{
    ast_expr_function::AstExprFunction, ast_local::AstLocal, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_local_function::AstStatLocalFunction, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatLocalFunction {
  pub fn new(
    location: Location,
    name: *mut AstLocal,
    func: *mut AstExprFunction,
    is_const: bool,
  ) -> Self {
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
      is_const,
    }
  }
}

pub fn ast_stat_local_function_ast_stat_local_function(
  location: Location,
  name: *mut AstLocal,
  func: *mut AstExprFunction,
  is_const: bool,
) -> AstStatLocalFunction {
  AstStatLocalFunction::new(location, name, func, is_const)
}
