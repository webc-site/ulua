pub struct CommaSeparatorInserter {
  pub(crate) first: bool,
  pub(crate) comma_position: *const Position,
}

impl CommaSeparatorInserter {
  pub fn new(_writer: &mut dyn Writer, comma_position: *const Position) -> Self {
    Self {
      first: true,
      comma_position,
    }
  }
}
use crate::records::{position::Position, writer::Writer};
