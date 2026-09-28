use crate::records::{
  ast_type_list::AstTypeList, ast_type_pack::AstTypePack,
  ast_type_pack_explicit::AstTypePackExplicit, location::Location,
};

impl_ast_node_new!(AstTypePackExplicit, AstTypePack, location: Location, type_list: AstTypeList);
