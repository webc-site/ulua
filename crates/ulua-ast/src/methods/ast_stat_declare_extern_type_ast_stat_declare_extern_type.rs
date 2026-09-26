use core::ptr::NonNull;

use crate::{
  functions::optional_node::opt_node,
  records::{
    ast_array::AstArray, ast_declared_extern_type_property::AstDeclaredExternTypeProperty,
    ast_name::AstName, ast_stat::AstStat, ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_table_indexer::AstTableIndexer, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatDeclareExternType {
  /// `indexer` 为 `None` 即 cpp 的 `nullptr`（Parser.cpp:1916 初始值）：extern type 没有索引签名。
  pub fn new(
    location: Location,
    name: AstName,
    super_name: Option<AstName>,
    props: AstArray<AstDeclaredExternTypeProperty>,
    indexer: Option<NonNull<AstTableIndexer>>,
  ) -> Self {
    Self {
      base: AstStat::new(<Self as AstNodeClass>::CLASS_INDEX, location),
      name,
      super_name,
      props,
      indexer: opt_node(indexer),
    }
  }
}
