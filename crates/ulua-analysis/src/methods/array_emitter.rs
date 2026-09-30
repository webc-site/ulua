//! `array_emitter` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::{methods::object_emitter_write_pair::WriteJson, records::array_emitter::ArrayEmitter};

impl Drop for ArrayEmitter<'_> {
  fn drop(&mut self) {
    self.finish();
  }
}

impl ArrayEmitter<'_> {
  pub fn finish(&mut self) {
    if self.finished {
      return;
    }
    let emitter = &mut *self.emitter;
    emitter.write_raw_string_view("]");
    emitter.pop_comma(self.comma);
    self.finished = true;
  }
}

// Source: `Analysis/include/Luau/JsonEmitter.h` (lines 190-200, hand-ported)
//
// C++ template:
// ```cpp
// template<typename T>
// void writeValue(T value)
// {
// if (finished) return;
// emitter->writeComma();
// write(*emitter, value);
// }
// ```

impl ArrayEmitter<'_> {
  /// `writeValue(T value)`
  pub fn write_value<T: WriteJson>(&mut self, value: T) {
    if self.finished {
      return;
    }

    let emitter = &mut *self.emitter;
    emitter.write_comma();
    // write(*emitter, value)
    value.write_json(emitter);
  }
}
