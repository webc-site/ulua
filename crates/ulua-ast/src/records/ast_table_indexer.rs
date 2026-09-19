use std::ptr::null_mut;

use crate::{
  enums::ast_table_access::AstTableAccess,
  records::{ast_type::AstType, location::Location},
};

#[derive(Debug, Clone)]
pub struct AstTableIndexer {
  pub index_type: *mut AstType,
  pub result_type: *mut AstType,
  pub location: Location,
  pub access: AstTableAccess,
  pub access_location: Option<Location>,
}

impl Default for AstTableIndexer {
  fn default() -> Self {
    Self {
      index_type: null_mut(),
      result_type: null_mut(),
      location: Location::default(),
      access: AstTableAccess::ReadWrite,
      access_location: None,
    }
  }
}
