use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_assign::AstStatAssign, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatAssign {
  pub fn new(
    location: Location,
    vars: AstArray<*mut AstExpr>,
    values: AstArray<*mut AstExpr>,
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
    }
  }
}

pub fn ast_stat_assign_ast_stat_assign(
  location: Location,
  vars: AstArray<*mut AstExpr>,
  values: AstArray<*mut AstExpr>,
) -> AstStatAssign {
  AstStatAssign::new(location, vars, values)
}
