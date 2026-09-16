use crate::{
  records::{ast_name::AstName, ast_node::AstNode, ast_type::AstType},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstGenericType {
  pub base: AstNode,
  pub name: AstName,
  pub default_value: *mut AstType,
}

impl AstNodeClass for AstGenericType {
  const CLASS_INDEX: i32 = ast_rtti_index("AstGenericType");
}
