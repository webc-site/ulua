use crate::records::{ast_type_list::AstTypeList, ast_type_pack::AstTypePack};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypePackExplicit {
  pub base: AstTypePack,
  pub type_list: AstTypeList,
}
