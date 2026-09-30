use crate::records::{ast_name::AstName, ast_stat::AstStat, ast_type::AstType, location::Location};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstStatDeclareGlobal {
  pub base: AstStat,
  pub name: AstName,
  pub name_location: Location,
  pub type_: *mut AstType,
}
