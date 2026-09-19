use crate::{
  records::{cst_expr_type_assertion::CstExprTypeAssertion, cst_node::CstNode, position::Position},
  rtti::CstNodeClass,
};

impl CstExprTypeAssertion {
  pub fn new(op_position: Position) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      op_position,
    }
  }
}
