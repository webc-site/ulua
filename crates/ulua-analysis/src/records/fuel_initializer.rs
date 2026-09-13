use crate::records::normalizer::Normalizer;

#[derive(Debug, Clone)]
pub struct FuelInitializer {
  pub(crate) normalizer: *mut Normalizer,
  pub(crate) initialized_fuel: bool,
}

impl Drop for FuelInitializer {
  fn drop(&mut self) {
    self.fuel_initializer_fuel_initializer_destructor();
  }
}
