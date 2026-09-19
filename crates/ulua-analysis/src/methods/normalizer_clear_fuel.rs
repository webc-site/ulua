use crate::records::normalizer::Normalizer;

impl Normalizer {
  pub fn clear_fuel(&mut self) {
    self.fuel = None;
  }
}
