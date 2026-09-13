use std::panic::resume_unwind;

use crate::records::{normalizer::Normalizer, normalizer_hit_limits::NormalizerHitLimits};
impl Normalizer {
  pub fn consume_fuel(&mut self) {
    if let Some(fuel) = self.fuel.as_mut() {
      *fuel -= 1;
      if *fuel <= 0 {
        resume_unwind(Box::new(NormalizerHitLimits));
      }
    }
  }
}
