use crate::records::array_emitter::ArrayEmitter;

impl Drop for ArrayEmitter<'_> {
  fn drop(&mut self) {
    self.finish();
  }
}
