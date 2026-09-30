use core::ptr::NonNull;

use crate::records::{ast_name::AstName, ast_node::AstNode, ast_type_pack::AstTypePack};

#[repr(C)]
#[derive(Debug)]
pub struct AstGenericTypePack {
  pub base: AstNode,
  pub name: AstName,
  pub default_value: Option<NonNull<AstTypePack>>,
}
