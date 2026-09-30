use crate::records::{
  ast_expr_function::AstExprFunction, ast_name::AstName, ast_stat::AstStat,
  ast_stat_type_function::AstStatTypeFunction, location::Location,
};

impl_ast_node_new!(
  AstStatTypeFunction,
  AstStat,
  location: Location,
  name: AstName,
  name_location: Location,
  body: *mut AstExprFunction,
  exported: bool,
  has_errors: bool,
);
