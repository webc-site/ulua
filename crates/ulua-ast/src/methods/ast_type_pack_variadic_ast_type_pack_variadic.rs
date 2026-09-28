use crate::records::{
  ast_type::AstType, ast_type_pack::AstTypePack, ast_type_pack_variadic::AstTypePackVariadic,
  location::Location,
};

impl_ast_node_new!(
  AstTypePackVariadic,
  AstTypePack,
  location: Location,
  variadic_type: *mut AstType,
);
