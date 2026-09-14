use crate::{
  records::{ast_array::AstArray, cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

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

impl CstNodeClass for CstExprTable {
  const CLASS_INDEX: i32 = ast_rtti_index("CstExprTable");
}

// C++ nested `CstExprTable::Separator` / `CstExprTable::Item` (Rust has no
// inherent associated types — these live at module scope).
pub type Separator = CstExprTableSeparator;
pub type Item = CstExprTableItem;

impl CstExprTable {
  pub const COMMA: CstExprTableSeparator = CstExprTableSeparator::Comma;
  pub const SEMICOLON: CstExprTableSeparator = CstExprTableSeparator::Semicolon;
  pub const MISSING: CstExprTableSeparator = CstExprTableSeparator::Missing;
}
