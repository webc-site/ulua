use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_return::AstStatReturn, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatReturn {
  pub fn new(location: Location, list: AstArray<*mut AstExpr>) -> Self {
    Self {
      base: AstStat {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
        has_semicolon: false,
      },
      list,
    }
  }
}

pub fn ast_stat_return_ast_stat_return(
  location: Location,
  list: AstArray<*mut AstExpr>,
) -> AstStatReturn {
  AstStatReturn::new(location, list)
}
