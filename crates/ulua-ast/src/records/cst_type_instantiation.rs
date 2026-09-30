use crate::records::{ast_array::AstArray, position::Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CstTypeInstantiation {
  pub left_arrow_1_position: Position,
  pub left_arrow_2_position: Position,

  pub comma_positions: AstArray<Position>,

  pub right_arrow_1_position: Position,
  pub right_arrow_2_position: Position,
}

impl Default for CstTypeInstantiation {
  fn default() -> Self {
    Self {
      left_arrow_1_position: Position::missing(),
      left_arrow_2_position: Position::missing(),
      comma_positions: AstArray::EMPTY,
      right_arrow_1_position: Position::missing(),
      right_arrow_2_position: Position::missing(),
    }
  }
}
