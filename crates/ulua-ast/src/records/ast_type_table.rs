use crate::records::{
  ast_array::AstArray, ast_table_indexer::AstTableIndexer, ast_table_prop::AstTableProp,
  ast_type::AstType,
};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypeTable {
  pub base: AstType,
  pub props: AstArray<AstTableProp>,
  pub indexer: *mut AstTableIndexer,
}
