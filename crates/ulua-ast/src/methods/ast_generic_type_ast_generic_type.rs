use core::ptr::NonNull;

use crate::records::{
  ast_generic_type::AstGenericType, ast_name::AstName, ast_node::AstNode, ast_type::AstType,
  location::Location,
};

impl_ast_node_new!(
  AstGenericType,
  AstNode,
  location: Location,
  name: AstName,
  default_value: Option<NonNull<AstType>>,
);
