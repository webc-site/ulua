use crate::records::{
  ast_array::AstArray, ast_table_indexer::AstTableIndexer, ast_table_prop::AstTableProp,
  ast_type::AstType, node_handle::OptNode,
};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypeTable {
  pub base: AstType,
  pub props: AstArray<AstTableProp>,
  /// 索引器槽位：cpp `AstTableIndexer* = nullptr` 表示无索引签名
  /// （`parseTableType` 仅首个 `[` 项建索引器），句柄化后空槽即 `None`。
  pub indexer: OptNode<AstTableIndexer>,
}
