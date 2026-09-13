use crate::{
  records::{
    ast_array::AstArray, ast_node::AstNode, ast_table_indexer::AstTableIndexer,
    ast_table_prop::AstTableProp, ast_type::AstType, ast_type_table::AstTypeTable,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstTypeTable {
  pub fn new(
    location: Location,
    props: AstArray<AstTableProp>,
    indexer: *mut AstTableIndexer,
  ) -> Self {
    Self {
      base: AstType {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      props,
      indexer,
    }
  }
}

pub fn ast_type_table_ast_type_table(
  location: Location,
  props: AstArray<AstTableProp>,
  indexer: *mut AstTableIndexer,
) -> AstTypeTable {
  AstTypeTable::new(location, props, indexer)
}
