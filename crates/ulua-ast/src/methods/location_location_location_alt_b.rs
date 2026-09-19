//! `Location::Location(const Position& begin, const Position& end)` — Location.h:74.

use crate::records::{location::Location, position::Position};

impl Location {
  pub fn new(begin: Position, end: Position) -> Location {
    Location { begin, end }
  }
}
