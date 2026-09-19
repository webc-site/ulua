use crate::{
  records::{
    ast_array::AstArray, ast_table_indexer::AstTableIndexer, ast_table_prop::AstTableProp,
    ast_type::AstType,
  },
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypeTable {
  pub base: AstType,
  pub props: AstArray<AstTableProp>,
  pub indexer: *mut AstTableIndexer,
}

impl AstNodeClass for AstTypeTable {
  const CLASS_INDEX: i32 = ast_rtti_index("AstTypeTable");
}
