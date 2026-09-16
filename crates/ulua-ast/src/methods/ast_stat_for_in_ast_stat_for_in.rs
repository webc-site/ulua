use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_local::AstLocal, ast_node::AstNode,
    ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_stat_for_in::AstStatForIn,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatForIn {
  pub fn new(
    location: Location,
    vars: AstArray<*mut AstLocal>,
    values: AstArray<*mut AstExpr>,
    body: *mut AstStatBlock,
    has_in: bool,
    in_location: Location,
    has_do: bool,
    do_location: Location,
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
      body,
      has_in,
      in_location,
      has_do,
      do_location,
    }
  }
}

pub fn ast_stat_for_in_ast_stat_for_in(
  location: Location,
  vars: AstArray<*mut AstLocal>,
  values: AstArray<*mut AstExpr>,
  body: *mut AstStatBlock,
  has_in: bool,
  in_location: Location,
  has_do: bool,
  do_location: Location,
) -> AstStatForIn {
  AstStatForIn::new(
    location,
    vars,
    values,
    body,
    has_in,
    in_location,
    has_do,
    do_location,
  )
}
