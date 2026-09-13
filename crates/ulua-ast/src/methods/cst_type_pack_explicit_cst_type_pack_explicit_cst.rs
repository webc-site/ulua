use crate::{
  records::{
    ast_array::AstArray, cst_node::CstNode, cst_type_pack_explicit::CstTypePackExplicit,
    position::Position,
  },
  rtti::CstNodeClass,
};

impl CstTypePackExplicit {
  pub fn new() -> Self {
    Self {
      base: CstNode::new(<Self as CstNodeClass>::CLASS_INDEX),
      open_parentheses_position: Position::missing(),
      close_parentheses_position: Position::missing(),
      comma_positions: AstArray::default(),
    }
  }
}

impl Default for CstTypePackExplicit {
  fn default() -> Self {
    Self::new()
  }
}
