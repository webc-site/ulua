use ulua_common::fint::LuauGenericCounterMaxSteps;

use crate::records::generic_counter::GenericCounter;

impl GenericCounter {
  pub fn check_limits(&mut self) {
    self.steps += 1;
    // FInt::LuauGenericCounterMaxSteps access（cpp Generalization.cpp:21，默认 1500）
    if self.steps > LuauGenericCounterMaxSteps.get() {
      self.hit_limits = true;
    }
  }
}
