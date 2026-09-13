use crate::{
  records::{
    ast_array::AstArray,
    cst_expr_table::{CstExprTable, CstExprTableItem},
    cst_node::CstNode,
  },
  rtti::CstNodeClass,
};

impl CstExprTable {
  pub fn new(items: AstArray<CstExprTableItem>) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      items,
    }
  }
}

pub fn cst_expr_table_cst_expr_table(items: AstArray<CstExprTableItem>) -> CstExprTable {
  CstExprTable::new(items)
}
