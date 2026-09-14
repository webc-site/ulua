use crate::records::fuel_initializer::FuelInitializer;

impl FuelInitializer {
  pub fn fuel_initializer_fuel_initializer_destructor(&mut self) {
    if self.initialized_fuel {
      unsafe { (*self.normalizer).clear_fuel() };
      self.initialized_fuel = false;
    }
  }
}
