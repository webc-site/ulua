use ulua_ast::records::position::Position;

use crate::records::warning_comparator::WarningComparator;

impl WarningComparator {
  #[inline]
  pub fn compare_position_position(&self, lhs: &Position, rhs: &Position) -> i32 {
    if lhs.line != rhs.line {
      return if lhs.line < rhs.line { -1 } else { 1 };
    }
    if lhs.column != rhs.column {
      return if lhs.column < rhs.column { -1 } else { 1 };
    }
    0
  }
}
