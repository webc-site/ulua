use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_local::AstLocal, ast_node::AstNode,
    ast_stat::AstStat, ast_stat_local::AstStatLocal, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatLocal {
  pub fn new(
    location: Location,
    vars: AstArray<*mut AstLocal>,
    values: AstArray<*mut AstExpr>,
    equals_sign_location: Option<Location>,
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
      vars,
      values,
      is_const,
      is_exported: false,
      equals_sign_location,
    }
  }
}

pub fn ast_stat_local_ast_stat_local(
  location: Location,
  vars: AstArray<*mut AstLocal>,
  values: AstArray<*mut AstExpr>,
  equals_sign_location: Option<Location>,
  is_const: bool,
) -> AstStatLocal {
  AstStatLocal::new(location, vars, values, equals_sign_location, is_const)
}
