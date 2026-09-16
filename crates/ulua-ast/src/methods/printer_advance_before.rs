use crate::records::{position::Position, printer::Printer, writer::Writer};

impl<'a, W: Writer> Printer<'a, W> {
  pub fn advance_before(&mut self, new_pos: Position, token_length: u32) {
    if new_pos.column >= token_length {
      let new_pos = Position::new(new_pos.line, new_pos.column - token_length);
      self.advance(new_pos);
    } else {
      self.advance(new_pos);
    }
  }
}
