use crate::{
  records::{ast_array::AstArray, cst_node::CstNode, position::Position},
  rtti::CstNodeClass,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypePackExplicit {
  pub base: CstNode,
  pub open_parentheses_position: Position,
  pub close_parentheses_position: Position,
  pub comma_positions: AstArray<Position>,
}

impl_cst_node_class!(CstTypePackExplicit);

impl CstTypePackExplicit {
  pub fn new() -> Self {
    Self {
      base: CstNode::new(<Self as CstNodeClass>::CLASS_INDEX),
      open_parentheses_position: Position::missing(),
      close_parentheses_position: Position::missing(),
      comma_positions: AstArray::EMPTY,
    }
  }
}

impl Default for CstTypePackExplicit {
  fn default() -> Self {
    Self::new()
  }
}

impl CstTypePackExplicit {
  /// cpp `CstTypePackExplicit(Position, Position, AstArray<Position>)` 带参重载
  /// （无参形态即 `new()`/`Default`）
  pub fn with_positions(
    open_parentheses_position: Position,
    close_parentheses_position: Position,
    comma_positions: AstArray<Position>,
  ) -> Self {
    Self {
      base: CstNode::new(<Self as CstNodeClass>::CLASS_INDEX),
      open_parentheses_position,
      close_parentheses_position,
      comma_positions,
    }
  }
}
