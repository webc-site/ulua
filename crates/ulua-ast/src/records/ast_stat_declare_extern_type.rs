use crate::{
  records::{
    ast_array::AstArray, ast_declared_extern_type_property::AstDeclaredExternTypeProperty,
    ast_name::AstName, ast_stat::AstStat, ast_table_indexer::AstTableIndexer,
  },
  rtti::{AstNodeClass, ast_rtti_index},
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

impl AstNodeClass for AstStatDeclareExternType {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatDeclareExternType");
}
