#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstExprTable {
  pub base: AstExpr,
  pub items: AstArray<Item>,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Item {
  pub kind: ItemKind,
  pub key: *mut AstExpr,
  pub value: *mut AstExpr,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemKind {
  List = 0,
  Record = 1,
  General = 2,
}

use crate::records::{ast_array::AstArray, ast_expr::AstExpr};
