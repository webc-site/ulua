use crate::{
  records::{
    ast_array::AstArray, ast_declared_extern_type_property::AstDeclaredExternTypeProperty,
    ast_name::AstName, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_declare_extern_type::AstStatDeclareExternType, ast_table_indexer::AstTableIndexer,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatDeclareExternType {
  pub fn new(
    location: Location,
    name: AstName,
    super_name: Option<AstName>,
    props: AstArray<AstDeclaredExternTypeProperty>,
    indexer: *mut AstTableIndexer,
  ) -> Self {
    Self {
      base: AstStat {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
        has_semicolon: false,
      },
      name,
      super_name,
      props,
      indexer,
    }
  }
}

pub fn ast_stat_declare_extern_type_ast_stat_declare_extern_type(
  location: Location,
  name: AstName,
  super_name: Option<AstName>,
  props: AstArray<AstDeclaredExternTypeProperty>,
  indexer: *mut AstTableIndexer,
) -> AstStatDeclareExternType {
  AstStatDeclareExternType::new(location, name, super_name, props, indexer)
}
