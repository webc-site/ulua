use crate::records::{position::Position, printer::Printer, writer::Writer};

impl<'a, W: Writer> Printer<'a, W> {
  pub fn maybe_advance_and_write(&mut self, pos: &Position, s: &str, always_write: bool) {
    if pos.has_value() {
      self.advance(pos);
      self.writer.write(s);
    } else if always_write {
      self.writer.write(s);
    }
  }
}
