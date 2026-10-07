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
