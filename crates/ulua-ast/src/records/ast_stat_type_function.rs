use crate::records::{
  ast_expr_function::AstExprFunction, ast_name::AstName, ast_stat::AstStat, location::Location,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstStatTypeFunction {
  pub base: AstStat,
  pub name: AstName,
  pub name_location: Location,
  pub body: *mut AstExprFunction,
  pub exported: bool,
  pub has_errors: bool,
}
