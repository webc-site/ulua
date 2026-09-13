use crate::records::{location::Location, position::Position};

impl Location {
  pub fn contains(&self, p: Position) -> bool {
    self.begin <= p && p < self.end
  }
}
