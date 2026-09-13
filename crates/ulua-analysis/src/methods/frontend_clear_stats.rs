use crate::records::frontend::Frontend;

impl Frontend {
  pub fn clear_stats(&mut self) {
    self.stats = Default::default();
  }
}
