use crate::{
  records::{
    ast_array::AstArray,
    cst_node::CstNode,
    cst_type_table::{CstTypeTable, CstTypeTableItem},
  },
  rtti::CstNodeClass,
};

impl CstTypeTable {
  pub fn new(items: AstArray<CstTypeTableItem>, is_array: bool) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      items,
      is_array,
    }
  }
}
