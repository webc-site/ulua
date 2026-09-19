use crate::records::{arg_name_inserter::ArgNameInserter, writer::Writer};

impl<'w, 'n, W: Writer> ArgNameInserter<'w, 'n, W> {
  pub fn operator_call(&mut self) {
    // 元素切片安全读：idx 越界或名字为空时跳过（解析器保证 idx < names.len，
    // cpp 侧 `data[i]` 同样依赖该不变式）。
    if let Some(name_val) = self.names.get(self.idx).copied().flatten() {
      // std::pair<AstName, Location> 在移植后为元组 (AstName, Location)：
      // name_val.first -> name_val.0, name_val.second -> name_val.1
      self.writer.advance(&name_val.1.begin);
      self.writer.identifier(name_val.0.as_bytes());

      if let Some(colon) = self.colon_positions.get(self.idx) {
        self.writer.advance(colon);
      }

      self.writer.symbol(":");
    }
    self.idx += 1;
  }
}
