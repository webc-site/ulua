use crate::records::{
  ast_array::AstArray, ast_table_indexer::AstTableIndexer, ast_table_prop::AstTableProp,
  ast_type::AstType, ast_type_table::AstTypeTable, location::Location,
};

impl_ast_node_new!(
  AstTypeTable,
  AstType,
  location: Location,
  props: AstArray<AstTableProp>,
  indexer: *mut AstTableIndexer,
);
