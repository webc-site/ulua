use crate::{
  records::{ast_array::AstArray, ast_expr::AstExpr, ast_local::AstLocal, ast_stat::AstStat},
  type_aliases::ast_class_member::AstClassMember,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstStatClass {
  pub base: AstStat,
  pub name: *mut AstLocal,
  pub super_: *mut AstExpr,
  pub members: AstArray<AstClassMember>,
  pub exported: bool,
  pub open: bool,
}
