use crate::records::{
  ast_array::AstArray, cst_expr_constant_string::CstExprConstantString,
  cst_expr_table::CstExprTableSeparator, cst_node::CstNode, position::Position,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypeTable {
  pub base: CstNode,
  pub items: AstArray<CstTypeTableItem>,
  pub is_array: bool,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypeTableItem {
  pub kind: CstTypeTableItemKind,
  pub indexer_open_position: Position,
  pub indexer_close_position: Position,
  pub colon_position: Position,
  pub separator: CstExprTableSeparator,
  pub separator_position: Position,
  pub string_info: *mut CstExprConstantString,
  pub string_position: Position,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CstTypeTableItemKind {
  Indexer,
  Property,
  StringProperty,
}

impl_cst_node_class!(CstTypeTable);
impl_cst_node_new!(CstTypeTable, items: AstArray<CstTypeTableItem>, is_array: bool);
