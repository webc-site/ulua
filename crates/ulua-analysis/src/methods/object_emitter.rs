//! `object_emitter` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::records::object_emitter::ObjectEmitter;

impl ObjectEmitter<'_> {
  pub fn finish(&mut self) {
    if self.finished {
      return;
    }
    let emitter = &mut *self.emitter;
    emitter.write_raw_string_view("}");
    emitter.pop_comma(self.comma);
    self.finished = true;
  }
}

impl Drop for ObjectEmitter<'_> {
  fn drop(&mut self) {
    self.finish();
  }
}
