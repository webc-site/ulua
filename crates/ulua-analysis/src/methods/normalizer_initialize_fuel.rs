use ulua_common::fint;

use crate::records::normalizer::Normalizer;
impl Normalizer {
  pub fn initialize_fuel(&mut self) -> bool {
    if self.fuel.is_some() {
      return false;
    }

    self.fuel = Some(fint::LuauNormalizerInitialFuel.get());
    true
  }
}
