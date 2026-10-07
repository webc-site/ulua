use crate::records::{
  ast_name::AstName, ast_stat::AstStat, ast_stat_declare_global::AstStatDeclareGlobal,
  ast_type::AstType, location::Location,
};

impl_ast_node_new!(
  AstStatDeclareGlobal,
  AstStat,
  location: Location,
  name: AstName,
  name_location: Location,
  type_: *mut AstType,
);
