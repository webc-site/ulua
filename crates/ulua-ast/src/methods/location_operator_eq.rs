impl Location {
  pub fn operator_eq(&self, rhs: &Self) -> bool {
    self.begin == rhs.begin && self.end == rhs.end
  }
}
use crate::records::location::Location;
