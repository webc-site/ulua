use crate::records::{comma_separator_inserter::CommaSeparatorInserter, writer::Writer};

impl CommaSeparatorInserter<'_> {
  /// 写入下一个元素前的逗号分隔符；首个元素跳过，其后在 CST 给出的逗号
  /// 位置写入 `,`。切片逐个消耗（与 cpp 游标 `add(1)` 推进同序）；耗尽时
  /// 直接写——解析器保证逗号数与元素数一致，cpp 裸指针同样依赖该不变式。
  pub fn operator_call<W: Writer>(&mut self, writer: &mut W) {
    if self.first {
      self.first = false;
    } else {
      if let Some((pos, rest)) = self.comma_positions.split_first() {
        writer.advance(pos);
        self.comma_positions = rest;
      }
      writer.symbol(",");
    }
  }
}
