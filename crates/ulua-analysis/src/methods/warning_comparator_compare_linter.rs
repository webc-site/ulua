use ulua_ast::records::{location::Location, position::Position};

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

  #[inline]
  pub fn compare_location_location(&self, lhs: &Location, rhs: &Location) -> i32 {
    let c = self.compare_position_position(&lhs.begin, &rhs.begin);
    if c != 0 {
      return c;
    }

    let c = self.compare_position_position(&lhs.end, &rhs.end);
    if c != 0 {
      return c;
    }

    0
  }
}
