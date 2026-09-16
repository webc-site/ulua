use crate::records::array_emitter::ArrayEmitter;

impl ArrayEmitter {
  pub fn finish(&mut self) {
    if self.finished {
      return;
    }
    let emitter = unsafe { &mut *self.emitter };
    emitter.write_raw_string_view("]");
    emitter.pop_comma(self.comma);
    self.finished = true;
  }
}
