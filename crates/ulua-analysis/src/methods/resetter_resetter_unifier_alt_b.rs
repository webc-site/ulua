use crate::records::resetter::Resetter;

impl Drop for Resetter {
  fn drop(&mut self) {
    unsafe {
      *self.variance = self.old_value;
    }
  }
}
