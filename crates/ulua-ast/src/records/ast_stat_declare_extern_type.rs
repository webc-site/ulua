use crate::records::{
  ast_array::AstArray, ast_declared_extern_type_property::AstDeclaredExternTypeProperty,
  ast_name::AstName, ast_stat::AstStat, ast_table_indexer::AstTableIndexer, node_handle::OptNode,
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatDeclareExternType {
  pub base: AstStat,
  pub name: AstName,
  pub super_name: Option<AstName>,
  pub props: AstArray<AstDeclaredExternTypeProperty>,
  /// 索引器槽位：cpp `AstTableIndexer* = nullptr`（`Parser.cpp:1916` 初始值）表示
  /// extern type 无索引签名，句柄化后空槽即 `None`。
  pub indexer: OptNode<AstTableIndexer>,
}
