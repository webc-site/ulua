use crate::{
  records::{ast_type_list::AstTypeList, ast_type_pack::AstTypePack},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypePackExplicit {
  pub base: AstTypePack,
  pub type_list: AstTypeList,
}

impl AstNodeClass for AstTypePackExplicit {
  const CLASS_INDEX: i32 = ast_rtti_index("AstTypePackExplicit");
}
