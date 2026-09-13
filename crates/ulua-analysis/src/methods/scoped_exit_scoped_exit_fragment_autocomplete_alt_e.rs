use crate::records::scoped_exit::ScopedExit;

impl ScopedExit {
  pub fn destructor(&mut self) {
    if let Some(func) = self.func.take() {
      func();
    }
  }
}
