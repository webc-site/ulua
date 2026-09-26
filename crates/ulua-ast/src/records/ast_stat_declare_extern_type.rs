use crate::records::{
  ast_array::AstArray, ast_declared_extern_type_property::AstDeclaredExternTypeProperty,
  ast_name::AstName, ast_stat::AstStat, ast_table_indexer::AstTableIndexer,
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatDeclareExternType {
  pub base: AstStat,
  pub name: AstName,
  pub super_name: Option<AstName>,
  pub props: AstArray<AstDeclaredExternTypeProperty>,
  pub indexer: *mut AstTableIndexer,
}
