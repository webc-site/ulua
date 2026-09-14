use crate::records::{location::Location, printer::Printer, writer::Writer};

impl<'a, W: Writer> Printer<'a, W> {
  pub fn write_end(&mut self, loc: &Location) {
    let mut end_pos = loc.end;
    if end_pos.column >= 3 {
      end_pos.column -= 3;
    }
    self.advance(end_pos);
    self.writer.keyword("end");
  }
}
