use crate::records::{position::Position, printer::Printer};

impl<'a> Printer<'a> {
  pub fn maybe_advance_and_write(&mut self, pos: &Position, s: &str, always_write: bool) {
    if pos.has_value() {
      self.advance(pos);
      self.writer.write(s);
    } else if always_write {
      self.writer.write(s);
    }
  }
}
