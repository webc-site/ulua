use crate::records::{position::Position, printer::Printer, writer::Writer};

impl<'a, W: Writer> Printer<'a, W> {
  pub fn maybe_advance_and_write(&mut self, pos: &Position, s: &str, always_write: bool) {
    // 实参恒为编译期符号字面量；write 走字节通道，此处收口转换。
    if pos.has_value() {
      self.advance(pos);
      self.writer.write(s.as_bytes());
    } else if always_write {
      self.writer.write(s.as_bytes());
    }
  }
}
