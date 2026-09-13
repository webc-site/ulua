use crate::{
  enums::ast_table_access::AstTableAccess,
  records::{ast_name::AstName, ast_type::AstType, location::Location},
};

#[derive(Debug, Clone)]
pub struct AstTableProp {
  pub name: AstName,
  pub location: Location,
  pub r#type: *mut AstType,
  pub access: AstTableAccess,
  pub access_location: Option<Location>,
}
