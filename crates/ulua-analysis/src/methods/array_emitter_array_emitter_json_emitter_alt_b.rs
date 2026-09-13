use crate::records::array_emitter::ArrayEmitter;

impl Drop for ArrayEmitter {
  fn drop(&mut self) {
    self.finish();
  }
}
