use crate::records::{ast_array::AstArray, cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CstExprTableSeparator {
  Comma,
  Semicolon,
  Missing,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CstExprTableItem {
  pub indexer_open_position: Position,
  pub indexer_close_position: Position,
  pub equals_position: Position,
  pub separator: CstExprTableSeparator,
  pub separator_position: Position,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprTable {
  pub base: CstNode,
  pub items: AstArray<CstExprTableItem>,
}

impl_cst_node_class!(CstExprTable);
impl_cst_node_new!(CstExprTable, items: AstArray<CstExprTableItem>);

// C++ `CstExprTable::Separator` 的嵌套类型在 Rust 中即顶层 `CstExprTableSeparator`，
// 消费方直接导入具体类型名。
impl CstExprTable {
  pub const COMMA: CstExprTableSeparator = CstExprTableSeparator::Comma;
  pub const SEMICOLON: CstExprTableSeparator = CstExprTableSeparator::Semicolon;
  pub const MISSING: CstExprTableSeparator = CstExprTableSeparator::Missing;
}
