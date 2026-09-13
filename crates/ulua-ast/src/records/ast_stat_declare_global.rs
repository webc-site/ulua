use crate::{
  records::{ast_name::AstName, ast_stat::AstStat, ast_type::AstType, location::Location},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstStatDeclareGlobal {
  pub base: AstStat,
  pub name: AstName,
  pub name_location: Location,
  pub type_: *mut AstType,
}

impl AstNodeClass for AstStatDeclareGlobal {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatDeclareGlobal");
}
