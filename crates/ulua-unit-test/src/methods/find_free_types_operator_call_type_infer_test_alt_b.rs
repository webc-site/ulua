use crate::records::find_free_types::FindFreeTypes;

impl FindFreeTypes {
  pub fn operator_call_mut<T>(&mut self, _id: T) -> bool {
    self.found_one = true;
    false
  }
}
