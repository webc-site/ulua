use crate::records::object_emitter::ObjectEmitter;

impl Drop for ObjectEmitter {
  fn drop(&mut self) {
    self.finish();
  }
}
