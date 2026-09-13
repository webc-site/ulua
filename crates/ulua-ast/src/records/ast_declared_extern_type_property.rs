use core::ptr::{null, null_mut};

use crate::{
  enums::ast_table_access::AstTableAccess,
  records::{ast_name::AstName, ast_type::AstType, location::Location},
};

#[derive(Debug, Clone, Copy)]
pub struct AstDeclaredExternTypeProperty {
  pub name: AstName,
  pub name_location: Location,
  pub ty: *mut AstType,
  pub is_method: bool,
  pub location: Location,
  pub access: AstTableAccess,
}

impl Default for AstDeclaredExternTypeProperty {
  fn default() -> Self {
    Self {
      name: AstName { value: null() },
      name_location: Location::default(),
      ty: null_mut(),
      is_method: false,
      location: Location::default(),
      access: AstTableAccess::ReadWrite,
    }
  }
}
