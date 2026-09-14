use crate::{
  records::{
    ast_array::AstArray, cst_node::CstNode, cst_type_pack_explicit::CstTypePackExplicit,
    position::Position,
  },
  rtti::CstNodeClass,
};

impl CstTypePackExplicit {
  pub fn cst_type_pack_explicit_position_position_ast_array_position(
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

pub fn cst_type_pack_explicit_position_position_ast_array_position(
  open_parentheses_position: Position,
  close_parentheses_position: Position,
  comma_positions: AstArray<Position>,
) -> CstTypePackExplicit {
  CstTypePackExplicit::cst_type_pack_explicit_position_position_ast_array_position(
    open_parentheses_position,
    close_parentheses_position,
    comma_positions,
  )
}
