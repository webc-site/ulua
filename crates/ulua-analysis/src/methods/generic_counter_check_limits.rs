use crate::records::generic_counter::GenericCounter;

impl GenericCounter {
  pub fn check_limits(&mut self) {
    self.steps += 1;
    // FInt::LuauGenericCounterMaxSteps access
    if self.steps > 1000 {
      self.hit_limits = true;
    }
  }
}
