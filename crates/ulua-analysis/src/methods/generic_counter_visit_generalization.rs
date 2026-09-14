use crate::records::generic_counter::GenericCounter;

impl GenericCounter {
  pub fn visit_type_id(&mut self) -> bool {
    self.check_limits();
    !self.hit_limits
  }
}
