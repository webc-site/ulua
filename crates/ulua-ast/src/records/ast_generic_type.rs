use core::ptr::NonNull;

use crate::records::{ast_name::AstName, ast_node::AstNode, ast_type::AstType};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstGenericType {
  pub base: AstNode,
  pub name: AstName,
  pub default_value: Option<NonNull<AstType>>,
}
