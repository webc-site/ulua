use crate::records::scoped_set_gc_threshold::ScopedSetGcThreshold;

impl Drop for ScopedSetGcThreshold {
  fn drop(&mut self) {
    unsafe {
      if !self.global.is_null() {
        (*self.global).gc_threshold = self.original_threshold;
      }
    }
  }
}
