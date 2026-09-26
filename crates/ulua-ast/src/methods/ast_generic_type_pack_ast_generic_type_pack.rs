use core::ptr::NonNull;

use crate::records::{
  ast_generic_type_pack::AstGenericTypePack, ast_name::AstName, ast_node::AstNode,
  ast_type_pack::AstTypePack, location::Location,
};

impl_ast_node_new!(
  AstGenericTypePack,
  AstNode,
  location: Location,
  name: AstName,
  default_value: Option<NonNull<AstTypePack>>,
);
