use crate::{
  records::{
    ast_array::AstArray, cst_node::CstNode, cst_type_pack_explicit::CstTypePackExplicit,
    position::Position,
  },
  rtti::CstNodeClass,
};

impl CstTypePackExplicit {
  /// cpp `CstTypePackExplicit(Position, Position, AstArray<Position>)` 带参重载
  /// （无参形态即 `new()`/`Default`）
  pub fn with_positions(
    open_parentheses_position: Position,
    close_parentheses_position: Position,
    comma_positions: AstArray<Position>,
  ) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      open_parentheses_position,
      close_parentheses_position,
      comma_positions,
    }
  }
}
