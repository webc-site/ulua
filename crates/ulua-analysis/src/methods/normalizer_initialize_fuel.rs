use ulua_common::FInt;

use crate::records::normalizer::Normalizer;
impl Normalizer {
  pub fn initialize_fuel(&mut self) -> bool {
    if self.fuel.is_some() {
      return false;
    }

    self.fuel = Some(FInt::LuauNormalizerInitialFuel.get());
    true
  }
}
