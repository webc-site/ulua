use core::ffi::c_char;

use crate::{
  records::{ast_expr::AstExpr, ast_name::AstName, location::Location, position::Position},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstExprIndexName {
  pub base: AstExpr,
  pub expr: *mut AstExpr,
  pub index: AstName,
  pub index_location: Location,
  pub op_position: Position,
  pub op: c_char,
}

impl AstNodeClass for AstExprIndexName {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprIndexName");
}
