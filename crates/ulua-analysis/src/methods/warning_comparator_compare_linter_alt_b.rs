use ulua_ast::records::location::Location;

use crate::records::warning_comparator::WarningComparator;

impl WarningComparator {
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
