use crate::records::{comma_separator_inserter::CommaSeparatorInserter, writer::Writer};

impl CommaSeparatorInserter {
  /// 写入下一个元素前的逗号分隔符；首个元素跳过，其后在 CST 给出的
  /// 逗号位置写入 `,`（`comma_position` 为空时直接写）。
  pub fn operator_call(&mut self, writer: &mut dyn Writer) {
    if self.first {
      self.first = false;
    } else {
      if !self.comma_position.is_null() {
        unsafe {
          writer.advance(&*self.comma_position);
          self.comma_position = self.comma_position.add(1);
        }
      }
      writer.symbol(",");
    }
  }
}
