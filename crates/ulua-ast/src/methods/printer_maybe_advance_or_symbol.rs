use crate::records::{position::Position, printer::Printer, writer::Writer};

impl<'a, W: Writer> Printer<'a, W> {
  /// CST 位置存在时校准前进并写符号，CST 缺失时无条件写符号；CST 存在但
  /// 位置无值时不写。合并 cpp 的 `maybeAdvanceAndWrite(pos, s)` + else
  /// `writer.symbol(s)` 双分支（visualize 系列高频同款）。
  pub(crate) fn maybe_advance_or_symbol(&mut self, pos: Option<&Position>, s: &str) {
    match pos {
      Some(pos) if pos.has_value() => {
        self.advance(pos);
        self.writer.symbol(s);
      }
      None => self.writer.symbol(s),
      Some(_) => {}
    }
  }
}
