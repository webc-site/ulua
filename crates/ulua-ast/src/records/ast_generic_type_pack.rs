use crate::{
  records::{ast_name::AstName, ast_node::AstNode, ast_type_pack::AstTypePack},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstGenericTypePack {
  pub base: AstNode,
  pub name: AstName,
  pub default_value: *mut AstTypePack,
}

impl AstNodeClass for AstGenericTypePack {
  const CLASS_INDEX: i32 = ast_rtti_index("AstGenericTypePack");
}
