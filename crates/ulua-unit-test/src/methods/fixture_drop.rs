use crate::records::fixture::Fixture;

impl Drop for Fixture {
  fn drop(&mut self) {
    let _ = self.frontend.take();
  }
}
