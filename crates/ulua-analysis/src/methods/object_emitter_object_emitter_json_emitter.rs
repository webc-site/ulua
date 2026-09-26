use crate::records::object_emitter::ObjectEmitter;

impl Drop for ObjectEmitter<'_> {
  fn drop(&mut self) {
    self.finish();
  }
}
