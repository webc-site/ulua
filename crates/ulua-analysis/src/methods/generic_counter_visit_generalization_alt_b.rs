use crate::records::generic_counter::GenericCounter;

impl GenericCounter {
  pub fn visit_type_id_function_type(&mut self) -> bool {
    self.check_limits();
    false
  }
}
