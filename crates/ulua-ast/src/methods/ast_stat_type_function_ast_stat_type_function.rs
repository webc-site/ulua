use crate::{
  records::{
    ast_expr_function::AstExprFunction, ast_name::AstName, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_type_function::AstStatTypeFunction, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatTypeFunction {
  pub fn new(
    location: Location,
    name: AstName,
    name_location: Location,
    body: *mut AstExprFunction,
    exported: bool,
    has_errors: bool,
  ) -> Self {
    Self {
      base: AstStat {
        base: AstNode {
          class_index: Self::CLASS_INDEX,
          location,
        },
        has_semicolon: false,
      },
      name,
      name_location,
      body,
      exported,
      has_errors,
    }
  }
}
