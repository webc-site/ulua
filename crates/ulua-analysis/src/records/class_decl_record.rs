use core::ptr::{null, null_mut};

use ulua_ast::records::ast_stat_class::AstStatClass;
use ulua_common::records::dense_hash_table::DenseDefault;

use crate::type_aliases::type_id::TypeId;
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClassDeclRecord {
  pub data_decl: *mut AstStatClass,
  pub ty: TypeId,
  pub new_blocked_ty: Option<TypeId>,
}

impl DenseDefault for ClassDeclRecord {
  fn dense_default() -> Self {
    Self {
      data_decl: null_mut(),
      ty: null(),
      new_blocked_ty: None,
    }
  }
}
